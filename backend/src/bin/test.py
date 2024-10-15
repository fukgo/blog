import requests

url = "http://127.0.0.1:8001/"
token = "cXFxOjE3Mjg5OTQ5NzA6RXZQbVdOYm1NRlEyRlZwOHp5RGZRalh0M1RpQ0plbDNHcW1EM3AwQ3FMQT0="
headers = {
    "Authorization": f"Bearer {token}",
    "Host": "localhost:8001",
    "Connection": "keep-alive",
    "Cache-Control": "max-age=0",
    "Sec-CH-UA": "\"Google Chrome\";v=\"129\", \"Not=A?Brand\";v=\"8\", \"Chromium\";v=\"129\"",
    "Sec-CH-UA-Mobile": "?0",
    "Sec-CH-UA-Platform": "\"Windows\"",
    "Upgrade-Insecure-Requests": "1",
    "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/129.0.0.0 Safari/537.36",
    "Accept": "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7",
    "Sec-Fetch-Site": "none",
    "Sec-Fetch-Mode": "navigate",
    "Sec-Fetch-User": "?1",
    "Sec-Fetch-Dest": "document",
    "Accept-Encoding": "gzip, deflate, br, zstd",
    "Accept-Language": "zh-TW,zh;q=0.9,en-US;q=0.8,en;q=0.7,zh-CN;q=0.6",
    "Cookie": "Pycharm-ee4d3d8f=65a3496e-b672-4559-89f8-2b7648711f08; _ga=GA1.1.943074815.1720015079; _ga_R1FN4KJKJH=GS1.1.1720015078.1.1.1720016976.0.0.0; _pk_id.1.1fff=5e7c71e7256cf2ea.1727622383"
}
headers1 = {
        "Authorization": f"Bearer {token}",
}
proxy = {'http': '127.0.0.1:7890', 'https': '127.0.0.1:7890'}
response = requests.request(method="GET",url=url, headers=headers)
print(response)
print(response.status_code)
print(response.text)
