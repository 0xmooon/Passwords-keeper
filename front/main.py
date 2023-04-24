import requests
import hashlib

API_URL = "http://127.0.0.1:8000/api/"

AUTH_ENDPOINT = "auth"
REGISTER_ENDPOINT = "register"
REQUEST_VAULT_ENDPOINT = "get_vault"
UPDATE_VAULT_ENDPOINT = "update_vault"
UPDATE_KEY_ENDPOINT = "update_key"

def make_request(endpoint, headers):
    r = requests.get(API_URL + endpoint, headers=headers)
    return r.text


def main():
    pass

if __name__ == "__main__":
    main()