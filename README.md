# pocha.moe API(?)

Hi! I wrote this in Rust because I like Rust. Full URL is `https://api.pocha.moe/v1`.

# Endpoints

## GET `/version/{mod_name}/{bs_version}`

A replacement for TheBlackParrot's old API-ish thing to check Beat Saber mod versions. Only really works with [my fork of DumbRequestManager](https://rustlang.pocha.moe/DumbRequestManager), but if anyone else is maintaining a mod by them, feel free to contact me.

Returns a string containing the latest version for a mod, or validation errors if there are any.

## GET `/health`

Returns a JSON: 
```json
{
    "status": "ok"
}
```