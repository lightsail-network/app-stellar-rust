from application_client.stellar_command_sender import StellarCommandSender, Errors

from dataset import SignTxTestCases, MNEMONIC
import pytest
from utils import get_testcases_names
from stellar_sdk import Keypair
from stellar_sdk.utils import sha256
from ragger.error import ExceptionRAPDU


@pytest.mark.parametrize("test_name", get_testcases_names(SignTxTestCases))
def test_sign_tx(backend, scenario_navigator, device, navigator, test_name):
    keypair = Keypair.from_mnemonic_phrase(MNEMONIC, index=0)
    path = "m/44'/148'/0'"
    transaction = getattr(SignTxTestCases, test_name)()
    client = StellarCommandSender(backend)
    signature_base = transaction.signature_base()

    # Send the sign device instruction.
    # As it requires on-screen validation, the function is asynchronous.
    # It will yield the result when the navigation is done
    with client.sign_tx(path=path, transaction=signature_base):
        # Validate the on-screen request by performing the navigation appropriate for this device
        scenario_navigator.review_approve(
            test_name=f"test_sign_tx_{test_name}", custom_screen_text="Sign "
        )
    response = client.get_async_response().data

    expected_signature = keypair.sign(sha256(transaction.signature_base()))
    assert response == expected_signature


def test_sign_tx_reject(backend, scenario_navigator):
    path = "m/44'/148'/0'"
    transaction = SignTxTestCases.op_create_account()
    client = StellarCommandSender(backend)

    with pytest.raises(ExceptionRAPDU) as e:
        with client.sign_tx(path=path, transaction=transaction.signature_base()):
            scenario_navigator.review_reject()

    # Assert that we have received a refusal
    assert e.value.status == Errors.SW_DENY
    assert len(e.value.data) == 0
