[section]
[header]Is Linux too difficult for new users?[/header]
[n]
Short answer: No
<focus>
[img]./desktop.png[/img]
[comment]
my desktop running [link]https://www.gentoo.org/[,]Gentoo Linux[/link]
[/comment]
</focus>
[n]
[n]
Preface: 
[n]
When I say Linux, I refer to GNU/Linux,
[n]
"distro" with regards to Linux is essentially it's own operating system that 
still uses the [link]https://github.com/torvalds/linux[,]Linux Kernel[/link]
[n]
[n]
For many years I've used Windows 7 and Windows 10, sure, I may not be an expert
at them but I'm still a consumer regardless. My experience with them has been
awful, but, I did not realize this until after I switched to Linux. Using 
Windows imposes a bar of sorts on you, a bar that limits your freedom. As a 
self proclaimed "power user" I throuroughly enjoy customizing my system. Up 
until I made the switch to Linux, I never understood all the Reddit post of 
users flexing their different setups. I never understood why people cared about 
Linux in the first place. It seemed very inferior; as not all games I played ran 
on it, people memed about how difficult it was to use/install (looking at you
Arch), and worst of all the people who did use Linux were elitist about
something so seemingly silly.
[n]
[n]
So what does this have to do with how difficult it is to use Linux?
[n]
[n]
It turns out Linux is far easier to use than Windows, the larger issue is that
everyone who does not use Linux is trained to use Windows or MacOs. They are
used to dealing with the buggyness and quirks of Windows and don't think to try
to relearn all the poor habbits those operating systems have taught you. That
setence may seem condescending, but it is true for the most part. Obviously
certain things that are irksome on Windows/MacOs carries over to Linux, but
fundamentally there are changes that seem like such a basic necessity, it's
plain out strange Windows and in some cases MacOs lacks them. The issue people
face is not that Linux is too difficult to use, but it is difficult to learn.
Partly due to the poor documentation in some areas, a toxic community in others,
and a overall unfamiliarity with the Terminal.
[n]
[n]
Why is Linux so good then?
[n]
[n]
A short answer to your question would be, Package Managers, Freedom,
and integration. These are all very vauge buzzwords; so I'll elaborate on them a
bit. 
[n]
[n]
Package Managers:
[n]
- Ever see these type of memes?
<div class="tenor-gif-embed" data-postid="18671901" data-share-method="host" data-aspect-ratio="1.77778" data-width="100%"><a href="https://tenor.com/view/linux-trash-linuxbad-gif-18671901">Linux Trash GIF</a>from <a href="https://tenor.com/search/linux-gifs">Linux GIFs</a></div> <script type="text/javascript" async src="https://tenor.com/embed.js"></script>
[n]
- Installing programs is quite the opposite, if I were to want to install the
web browser [link]https://www.mozilla.org/en-US/firefox/new/[,]Firefox[/link] on
my laptop, I'd simply run this command, type in my password, then in a matter of
seconds Firefox would be a valid command and the gates for me to fill my brain
with useless memes from the internet would be wide open.
[codeHighlight][lang]bash[/lang]
doas pacman -S firefox

[/codeHighlight]
[comment]
pacman is the package manager for [link]https://archlinux.org/[,]Arch Linux[/link]
[/comment]
[n]
- As the name implies, a package manager manages packages. What encompasses a
package? Well, many things. It manages libraries, programs, depenencies, and
even fonts. The point of said package manager is a centralized program to
maintain, update, and install things FOR YOU. A computer is meant to make doing
work easier, among other things, in which the laborious process of installing,
maintaining, and updating programs on Windows/MacOs is quite the opposite.
Package managers come in many different forms, and they make up a large part of
the distribution of Linux that you use. Many programming languages are bundled
with their own package manager, examples: nodeJS/npm, Rust/Cargo, Python/Pip,
etc. BSD operating systems also use package managers.Why does the idea not carry 
over to Windows/MacOs? Well, it does, sortof. Both those OSes have sudo-unofficial 
package managers that get better every commit. 
[link]https://docs.microsoft.com/en-us/windows/package-manager/winget/[,]Winget[/link]
and [link]https://brew.sh/[,]Homebrew[/link] for Windows and MacOs respectively
attempt to fill this lack of convience. They are incomplete, which is more of a
lack of packages compared to the Linux alternatives than actual features. This
proves that a package manager can at least semi successfuly be done, but they
aren't. 
[n]
[n]
Freedom:
[n]
- A big concern of mine is when I use my computer, I want to know what it's
doing. Now many people don't care, why should they? That's completely fine, but
I want to know what my computer is doing, and if it's making network request, I
better know what they're for. Using Windows I would occasionally open task
manager and while nothing was running, a spike on my ethernet controller's tx/rx
statistics would pop up everyonce in a while. What were they doing? I had
cortona and weather disabled, I had pretty much all the extra bloat that they
call "features" disabled. Why was my computer making seemingly random network
request? Nobody outside of Microsoft knows, after all, all the source code is
hidden away in one of the hundreds or thousands of datacenters that Microsoft
owns. At the time I didn't care all that much, but it now is infuriating that I
could never know what Microsoft is sending out. Microsoft even admits to
[link]https://docs.microsoft.com/en-us/windows/privacy/configure-windows-diagnostic-data-in-your-organization[,]this[/link].
In which you can only disable the telemetry Windows Server, Windows Enterprise,
and Windows Education editions. How absurd, I will never understand why they do
not provide an option for "normal" users, even if it is opt out.
[n]
[n]
[n]
[btt-button]
