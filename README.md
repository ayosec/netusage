# netusage

Small tool to show network usage, collected from `/proc/net/dev`.

## Polybar

To use with Polybar:

```ini
[module/network]
type = custom/script
exec = netusage -i eth0 -u 3
tail = true
label =  %output%
```
