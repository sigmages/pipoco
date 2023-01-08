# -*- coding: utf-8 -*-
"""
-------------------------------------------------
   File Name：     xvideos
   Description :
   Author :       qiuqiu, (updated by ronald-tr)
   date：          2019/10/23
-------------------------------------------------
"""
import json
import random
import re
from html import unescape
from pydantic import BaseModel

import requests
from bs4 import BeautifulSoup


class XvideosException(Exception):
    pass


class Comment(BaseModel):
    title: str
    author: str
    content: str
    datediff: str
    country: str
    score: str

PATTERN = re.compile(r'/video(\d+)/.*')
N_ATTEMPTS = 5


def _fetch_page(url):
    proxies = {'http': '127.0.0.1:1080', 'https': '127.0.0.1:1080'}
    res = requests.get(url)

    if res.status_code != 200:
        raise Exception(f'Response Error: {res.status_code}')

    return BeautifulSoup(res.text, 'html.parser')


def _find_videos(soup):
    result = []

    for element in soup.select('.thumb-block > div > p > a'):
        # print(element)
        try:
            reference = PATTERN.match(element['href']).group(1)
        except AttributeError:
            pass
        result.append((reference, element['title']))
    return result


def _get_comments(title, video_ref):
    url_mask = 'https://www.xvideos.com/video-get-comments/{0}/0/'
    url_mask = 'https://www.xvideos.com/threads/video-comments/get-posts/top/{0}/0/0'
    url = url_mask.format(video_ref)
    proxies = {'http': '127.0.0.1:1080', 'https': '127.0.0.1:1080'}
    res = requests.post(url)

    if res.status_code != 200:
        raise Exception('Response Error: ' + str(res.status_code))

    posts = json.loads(res.text)['posts']

    if posts['nb_posts_total'] < 1:
        return

    def get_safe(prop):
        x = item.get(prop, '')
        return unescape(x) if x and isinstance(x, str) else None

    result = []
    for item in posts['posts'].values():
        try:
            score = item['votes']['nb'] - item['votes']['nbb']
        except (KeyError, TypeError):
            score = 0

        comment = Comment(
            title=title,
            author=get_safe('name') or 'Unknown user',
            content=get_safe('message'),
            datediff=get_safe('time_diff') or 'some time ago',
            country=get_safe('country_name') or 'unknown region',
            score=score
        )

        result.append(comment)

    return result


def choose_random_porn_comment(search_term=None):
    for _ in range(N_ATTEMPTS):
        r = random.randint(1, 10)

        if search_term:
            url = f'https://www.xvideos.com/?k={search_term}&p={r}'
        else:
            url = f'https://www.xvideos.com/'

        page = _fetch_page(url)

        references = _find_videos(page)
        if not references:
            msg = 'No videos were found'
            if search_term:
                msg += f' with search term "{search_term}"'
            raise XvideosException(msg + '. :(')

        choice_references = random.choice(references)
        comments = _get_comments(choice_references[1], choice_references[0])

        if not comments:
            continue

        return random.choice(comments)

    raise XvideosException(f'No comments were found after {N_ATTEMPTS} attempts. :(')


def choose_random_porn_comment_as_json(search_term=None):
    search_term = search_term if search_term != "" else None
    return choose_random_porn_comment(search_term).json()


def main():
    comment = choose_random_porn_comment()
    print(comment)
    print()


if __name__ == '__main__':
    main()
