<h2 align="center"> ━━━━━━  ❖  ━━━━━━ </h2>

<!-- BADGES -->
<div align="center">
   <p></p>
   
   <img src="https://img.shields.io/github/stars/dotzenith/AvatarAPI.rs?color=F8BD96&labelColor=302D41&style=for-the-badge">   

   <img src="https://img.shields.io/github/forks/dotzenith/AvatarAPI.rs?color=DDB6F2&labelColor=302D41&style=for-the-badge">   

   <img src="https://img.shields.io/github/actions/workflow/status/dotzenith/AvatarAPI.rs/deploy.yml?branch=main&color=89b4fa&labelColor=302D41&style=for-the-badge&label=Deployment"/>
   
   <img src="https://img.shields.io/github/actions/workflow/status/dotzenith/AvatarAPI.rs/test.yml?branch=main&color=ABE9B3&labelColor=302D41&style=for-the-badge&label=Tests"/>
   <br>
</div>

<p/>

---

### ❖ Information 

AvatarAPI, is a free API serving quotes from Avatar: The Last Airbender. The quotes are sourced from [AvatarQuotes](https://github.com/dotzenith/AvatarQuotes/) and the endpoints are available at https://avatarquotes.xyz/api/{endpoint} (See [Usage](#Usage) for the different endpoints)

<img src="https://github.com/dotzenith/dotzenith/blob/main/assets/AvatarAPI/AvatarAPI.png" alt="API response example">

---

### ❖ Usage

AvatarAPI has a few different endpoints for different needs but the usage is relatively the same for all of them. The examples use [httpie](https://httpie.io/) but anything that can make GET requests can be used as well

#### ❖ Fetch 5 random quotes 

Endpoint: `/api/quotes`

```
$ http GET https://avatarquotes.xyz/api/quotes
```

```json
{
    "num": 5,
    "quotes": [
        {
            "quote": "...",
            "character": "...",
            "nation": "...",
            "bending": "...",
            "episode": "...",
            "book": "...",
        },
        "...4 more"
    ]
}
```

<b></b>

#### ❖ Fetch 5 random quotes from any given column

Endpoints:
- `/api/quotes/character`
- `/api/quotes/nation`
- `/api/quotes/bending`
- `/api/quotes/episode`
- `/api/quotes/book`

```
$ http GET https://avatarquotes.xyz/api/quotes/character value==Aang
```

```json
{
    "num": 5,
    "quotes": [
        {
            "quote": "...",
            "character": "Aang",
            "nation": "...",
            "bending": "...",
            "episode": "...",
            "book": "...",
        },
        "...4 more"
    ]
}
```

All of the endpoints above require the `value` parameter, which is case sensitive

There is also an optional `num` parameter that can be used to specify how many quotes you'd like. Quotes can be in the range `[0, 255]` (yes it's just a `u8`, I don't know why you'd want 0 quotes)

<b></b>

#### ❖ Fetch all valid values for any given column

Endpoints:
- `/api/all/character`
- `/api/all/nation`
- `/api/all/bending`
- `/api/all/episode`
- `/api/all/book`

```
$ http GET https://avatarquotes.xyz/api/all/bending
```

```json
{
    "num": 6,
    "values": [
        "Water",
        "None",
        "Fire",
        "All",
        "Air",
        "Earth"
    ]
}
```

These endpoints can be helpful if you're trying to use the `quotes` endpoints but can't get the `value` right

<b></b>

#### ❖ The `response.num` field

The response from all of the endpoints includes the `response.num` field, which simply specifies the number of quotes returned. A natural question to ask is "Well why is that needed, shouldn't it just be 5 in the default case, or equal to `num` in case the `num` parameter was passed in?"

The answer is "well yes but no".

For example, some characters like Koh only have 2 quotes. So even without the `num` parameter, we run into some issues. This is why the API just returns as many quotes it can and updates the `response.num` field in the case that `num` is too large for the given request.

The `response.num` field exists as a sanity check. Users can confirm that they got the number of quotes they requested, or implement special logic in case it's different.

---

### ❖ Rate Limiting

AvatarAPI currently allows 100 requests per hour from a given IP address. In case you desire more, feel free to take a look at the [Self Hosting](#Self-Hosting) section.

---

### ❖ Self Hosting

AvatarAPI is relatively easy to self-host. The only requirements are [Docker](https://www.docker.com/), [Git](https://git-scm.com/) and pretty much any webserver, for example: [nginx](https://www.nginx.com/)

#### ❖ Clone repo with submodules and cd into it 

```
$ git clone --recurse-submodules https://github.com/dotzenith/AvatarAPI.rs.git
$ cd AvatarAPI
```

<b></b>

#### ❖ Build docker image

```
$ docker build -t avatarapi:latest .
```

#### ❖ Run the container

```
$ docker run -p 3000:3000 -d --name avatarapi avatarapi:latest
```

#### ❖ Set up a reverse proxy

After the step above, set up a reverse proxy using a webserver of your choice and enjoy your very own AvatarAPI :)

---

### ❖ What's New?

0.1.0 - Initial Release

---

<div align="center">

   <img src="https://img.shields.io/static/v1.svg?label=License&message=MIT&color=F5E0DC&labelColor=302D41&style=for-the-badge">

</div>
