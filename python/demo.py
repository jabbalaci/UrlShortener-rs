#!/usr/bin/env python3

"""
It served as a basis for the Rust implementation.
"""

import os
from pprint import pprint

import requests

API_KEY = os.environ.get('BITLY_ACCESS_TOKEN', '')
API_URL = "https://api-ssl.bit.ly/v4"


def shorten(url):
    shorten_url = f"{API_URL}/shorten"
    params = {"long_url": url}
    headers = {"Authorization": f"Bearer {API_KEY}"}
    response = requests.post(shorten_url, json=params, headers=headers)
    data = response.json()
    pprint(data)


def expand(url_id):
    expand_url = f"{API_URL}/expand"
    params = {"bitlink_id": url_id}
    headers = {"Authorization": f"Bearer {API_KEY}"}
    response = requests.post(expand_url, json=params, headers=headers)
    data = response.json()
    pprint(data)


def main():
    long_url = "https://index.hu"
    shorten(long_url)
    print("---")
    short_url_id = "bit.ly/2Pkk3cM"
    expand(short_url_id)

##############################################################################


if __name__ == "__main__":
    main()
