# Contributors

* [mGhassen](https://github.com/mGhassen)

```shell
p=1;
while true; do
    s=$(curl "https://api.github.com/repos/Guepard-Corp/Guepard/contributors?page=$p") || break
    [ "0" = $(echo $s | jq length) ] && break
    echo $s | jq -r '.[] | "* [" + .login + "](" + .html_url + ")"'
    p=$((p+1))
done
```