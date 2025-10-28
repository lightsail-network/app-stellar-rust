import pytest
from application_client.stellar_command_sender import Errors, StellarCommandSender
from ragger.error import ExceptionRAPDU
from stellar_sdk import Keypair
from stellar_sdk.utils import sha256
from utils import (
    ROOT_SCREENSHOT_PATH,
    SettingsId,
    configure_device_settings,
    get_testcases_names,
)

from dataset import MNEMONIC, SignSorobanAuthorizationTestCases


@pytest.mark.parametrize(
    "test_name", get_testcases_names(SignSorobanAuthorizationTestCases)
)
def test_sign_soroban_auth(backend, scenario_navigator, device, navigator, test_name):
    keypair = Keypair.from_mnemonic_phrase(MNEMONIC, index=0)
    path = "m/44'/148'/0'"
    preimage = getattr(SignSorobanAuthorizationTestCases, test_name)()
    client = StellarCommandSender(backend)

    with client.sign_soroban_auth(
        path=path, soroban_authorization=preimage.to_xdr_bytes()
    ):
        # Validate the on-screen request by performing the navigation appropriate for this device
        scenario_navigator.review_approve(
            ROOT_SCREENSHOT_PATH,
            test_name=f"test_sign_soroban_auth_{test_name}",
            custom_screen_text="Sign ",
        )
    response = client.get_async_response().data

    expected_signature = keypair.sign(sha256(preimage.to_xdr_bytes()))
    assert response == expected_signature


def test_sign_soroban_auth_with_nonce_enabled(
    backend, scenario_navigator, device, navigator
):
    keypair = Keypair.from_mnemonic_phrase(MNEMONIC, index=0)
    path = "m/44'/148'/0'"
    preimage = SignSorobanAuthorizationTestCases.soroban_auth_create_smart_contract()
    client = StellarCommandSender(backend)
    configure_device_settings(navigator, device, SettingsId.ENABLE_SEQUENCE_AND_NONCE)
    with client.sign_soroban_auth(
        path=path, soroban_authorization=preimage.to_xdr_bytes()
    ):
        # Validate the on-screen request by performing the navigation appropriate for this device
        scenario_navigator.review_approve(
            ROOT_SCREENSHOT_PATH,
            custom_screen_text="Sign ",
        )
    response = client.get_async_response().data

    expected_signature = keypair.sign(sha256(preimage.to_xdr_bytes()))
    assert response == expected_signature


def test_sign_soroban_auth_reject(backend, scenario_navigator):
    path = "m/44'/148'/0'"
    preimage = SignSorobanAuthorizationTestCases.soroban_auth_create_smart_contract()
    client = StellarCommandSender(backend)

    with pytest.raises(ExceptionRAPDU) as e:
        with client.sign_soroban_auth(
            path=path, soroban_authorization=preimage.to_xdr_bytes()
        ):
            scenario_navigator.review_reject(ROOT_SCREENSHOT_PATH)

    # Assert that we have received a refusal
    assert e.value.status == Errors.SW_DENY
    assert len(e.value.data) == 0
