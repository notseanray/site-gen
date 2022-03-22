<p align="center">static site generator</p>

<p align="center">
	<a href="./LICENSE"><img src="https://img.shields.io/badge/license-GPL%20v3.0-blue.svg"></a>
</p>

If you haven't checked it out in action yet, you can do so [here](https://notseanray.github.io/seanray.net/).

The goals of this project is fairly simple: 'transpile' SML (not to be confused with Standard ML, but my own version of ML) into pure html + css. I put transpile in quotes as this is far from any form of a proper compiler or transpiler. Much of the generator is just replacing certain strings with others, it can be thought of a short-hand of normal html in a way. 
* 'SML' stands for Sean Markdown Language in this instance, it's quite a silly name so eventually I'll replace it.

Due to the nature of ONLY replacing certain things and the lack of a lexer, tokenizer, etc. inline html has first class support. If things are expressed better in raw html, then it can be done interchangably with writing shorthand html. The project uses the [notify](https://docs.rs/notify/latest/notify/) crate in order to watch a certain directory for any file system updates. If any update is made, a check of the current hashes between files are made via [seahash](https://docs.rs/seahash/latest/seahash/index.html). This keeps 'compile' times down to a minimum (~3.5 ms on my PC) so changes can quickly be made. This can be used in tandem with the basic webserver [shttpd](https://github.com/notseanray/shttpd) to provide a decent development experience. Opening the html directly in your browser works well too.

All SCSS is compiled + minified into css with the [rsass](https://docs.rs/rsass/latest/rsass/) crate, syntax highlighting for some common languages is also featured courtesy of [highlightjs](https://highlightjs.org/).

##### Directory structure
* static - houses 'templates', or prebuilt files for certain pages
* repos - contains a python + shell script to fetch repository data from GitHub and format + display it
* content - this directory has all of the *actual* site data in the form of sml
* build - this directory is made after content has been filled with anything, contains the final html + css

##### templates

Templates are very useful for reducing the amount of markdown that needs to be written. The `all` template applies to all sml files and can append content to the beginning or end of a file, in my case I use it to import the font and show the navbar. The `[split]` tag can be used to indicate if it's in the top or bottom half, anything before the tag is inserted to the beginning of the final file while anything after is inserted at the end. This is the case for all templates. To apply templates to multiple files you simply name it a certain way. For example, the index template applies to any file that has index as the start of it's name. For blog post there will be a `post` template or something simlar. This is very versatile and allows for a really comfy coding experience.


##### components

These are a series of different tags that are currently hard coded in, they can provide a short hand to declaring html elements. Here's an example of this:
```
[codeHighlight][lang]bash[/lang]
echo "ssh-ed25519 \
AAAAC3NzaC1lZDI1NTE5AAAAIHhssm/d0+mNX2cAhohgwOCRBjYCIQzylzRD2Hwr8lrr \
sean@seanray.net" >> ~/.ssh/authorized_keys

[/codeHighlight]
```

is equal to writing:
```
<pre><code class="language-bash">
echo "ssh-ed25519 \
AAAAC3NzaC1lZDI1NTE5AAAAIHhssm/d0+mNX2cAhohgwOCRBjYCIQzylzRD2Hwr8lrr \
sean@seanray.net" >> ~/.ssh/authorized_keys

</code></pre>

```

In this particular instance it is not much less to write, but I'd say the syntax is more comfortable in general. The real benefit is when there are certain things that will always be the same across pages, for example, the utterances chat page will likely always be the same so that is premade to convert 1 line into 7. Again not a great improvement, but the idea behind it is, in my humble opinion, pretty cool.

##### Improvements
* files are read from the file system into a string then hashed every time there is an update, this can be improved.
* it's a 'fake' ML  spin off language, a real compiler will be a future addition that will be very awesome :)
* the projects page generation handled by the python script does not preserve the hand written project descriptions, due to this I had it backup everytime it was run which works but can be improved greatly.
* I originally wanted to bundle in a webserver to show updates without page reload, but that was a bit complex for me. It's worth looking into however.
* dynamically load 'components' from a file, right now it's pretty much hard coded but that is not great
* auto 'recompile' when the templates are changed, at the moment only things in the content directory invoke the compiler.

##### Why?

I wanted to create a website to post some of my thoughts about Linux, programming language design, and showcase my projects. I could've easily done this in React or Nextjs, but I thought writing my own *framework (it's not really a framework, but whatever)* would be a lot of fun and slightly more impressive. In it's current state this is a simple project but it's base to build on. That's the best thing about software development: there's always something to build off of and improve. :D
