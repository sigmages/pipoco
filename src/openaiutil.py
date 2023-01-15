import openai
import os

api_key = os.environ.get('OPENAI_API_KEY')

# Set the API key
openai.api_key = api_key
def get_image_link(prompt: str) -> str:
    openai.Image.create(prompt=prompt)['data'][0]['url']


def get_text_suggestion(prompt: str) -> str:
    response = openai.Completion.create(
        engine="text-davinci-002",
        prompt=prompt,
        temperature=0.5,
        max_tokens=2000,
        top_p=1,
        frequency_penalty=1,
        presence_penalty=1
    )['choices'][0]['text']
    return response

get_text_suggestion('write in python, a code that print hello word in japanese')
