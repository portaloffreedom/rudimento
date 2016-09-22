# rudimento
Wayland compositor in rust heavily inspired from weston (for the moment)

I'm doing this for fun, to learn more about both wayland and rust. Still, I wound't mind if it becomes something serious.

It will differ in:
- is written in rust and not C (more secure, but mostly for fun)
- it will use SSD instead of CSD, because CSD sucks (we don't want windows again). [Here](https://blog.martin-graesslin.com/blog/2010/05/open-letter-the-issues-with-client-side-window-decorations/) in detail most of the reasons why CSD is not a good choice.

I'm doing this on my laptop, which only has an NVIDIA card, so that will be the first backend I will support.
 
Still far from working, any help is appreciated. Especially on running it on hardware I don't have.
