# Chinfusor, a universal solution for reading texts in foreign alphabets on Linux

When one wants to learn a new language, or simply use a language other than his native, the very basic ability required in order to do so is to be able to read text written in the particular language. And while this is quite easy for latin-based languages, which can be read simply as they are, the situation gets much more complicated, when trying to read texts written in other alphabets.

Languages such as Mandarin Chinese, Japanese or Russian are widely used in the world. Taking in the fact, that population of China at time of writing this text makes about 1.43 billion, approximately every fifth human in the world speaks Chinese.\
And yet, when running into chinese characters, espeak, the default speech synthesizer for many Linux distributions says just "chinese character, chinese character, chinese character, ...".\
Russian's cyrillic or Japanese Kana don't produce much more informative results. That is a problem, especially if you want to read text in these languages, either because you want to learn or simply use them.

Thus, Chinfusor is here to help you.

## How does it work

Speech-dispatcher is a well-known Linux ekvivalent of Windows' sapi. It contains modular speech modules, which in turn represent various voices for various languages. Espeak-ng is one such a module, other common are its predecessor Espeak or Festival. Orca, the default gnome screenreader then uses speech-dispatcher to speak everything it needs to, without vorying about underlying details of speech synthesis.
Chinfusor is also a speech module for speech dispatcher. But instead of producing speech on its own, it internally loads other speech modules and after receiving text to speak, it separates parts written in different alphabets, and redirects the right text, to right engine, in right time, according to configuration.
That results not just in reading foreign alphabets correctly, but because all loading and configuration of other modules is handled by Chinfusor, user can configure specific speech parameters for each alphabet separately. If you're fluent in English, but your Russian still needs practice, you can simply slow the russian engine down, while going english at full speed.\
May be you're fine with espeak for your native language, but for tonal Chinese, you'd prefer a more human-like pronounciation. With Chinfusor, you can use different engines for different alphabets without any issues.

And all of that is possible right from your screenreader, without any extra work required after the initial setup. If you're woried about latency caused by the additional text parsing and forwards between processes, note that Chinfusor is programmed in [Rust](https://rust-lang.org/), a modern programming language, which is not just cool and memory-safe, but also blazingly fast, its top three priorities are safety, concurrency and speed.\
Rust source code is compiled directly into native code, so you should feel no friction when using Chinfusor, even in the wildest use scenarios.

## How to set it up

There are few steps required in order to make Chinfusor work correctly.

### Configuration

Although Chinfusor will stay with its defaults when you don't provide any configuration file, you will most likely want to set it up, so you can make changes quickly if required. Create a directory named chinfusor in ~/.config, you may need to enable showing hidden folders in order to access the directory.
Then, copy alphabets_settings.csv and settings.conf files, which are distributed along with the program, into your newly created chinfusor directory.

alphabets_settings.csv is a comma separated values type configuration file. It configures properties such as used speech module, pitch or rate for any alphabet defined by range in unicode table. Each line goes as follows (values separated with commas, without spaces):

* Alphabet, the alphabet, which is being configured. The name is purely informational and is not used by Chinfusor in any way, so you can select whatever title you like.
* Unicode ranges, ranges in unicode table specifying this alphabet. The format is u0xa-u0xb, where a and b stand for hexadecimal values. Without 0x, the number behind u is considered decimal. More ranges can be specified for one alphabet as once, in format u0xa-u0xbu0xc-u0xd, but it is recommended to separate each range with a delimiter for readability i.e. u0xa-u0xb+u0xc-u0xd. You can use whatever delimiter you like. Star sign (*) in this field or nothing in this field is considered to denote a latin alphabet. There should be exactly one latin alphabet specified. If more are found, the firstone is considered relevant, if none is found, default configuration will be used for the latin alphabet. Also note, that unicode ranges specified in the whole configuration musn't overlap. If they do, the behavior will be undefined.
* Module, the path to target speech module, should be absolute.
* Arg, the argument to pass to desired module, usually an absolute path to module's configuration file.
* Language, the language to be used for the selected alphabet, for example en, sk, ru etc.
* Voice, The voice to be used, typically male1, but other variants could be supported by various engines.
* Punctuation mode, the punctuation mode to be used, possible values are none, some and all.
* Pitch, the desired pitch for the selected alphabet. Possible values are from -100 to 100.
* Capitals pitch, the pitch to be used for capital letters of the selected alphabet, possible values range from -100 to 100.
* Rate, the speech rate to be used for the selected alphabet, possible values are from -100 to 100.
* Volume, the volume to be used for the selected alphabet, values range from -100 to 100.
* Firejailed, whether the speech module for the selected alphabet should be sandboxed, more in later section. Possible values are yes or true for enabling the feature, everything else is no.

A number sign (#) on start of a line denotes a comment, you can use it to comment your configuration.

Also note, that while there are some validity checks for correctness of entered values, they're not in any means advanced. For example, module's path or arg path is not checked for validity, so if you enter wrong values there, you can effectively break the program. Be careful in what you're doing, speech is a crucial part of our work with computers, so you don't want it broken.

settings.conf is a configuration file with simple structure, each line starts with a setting key, which is followed by colon, space and value. # on start of line again denotes comment.

Currently, just one configuration is available in this file, punctuation_characters. It specifies the characters considered as punctuation while parsing text. \\n and \\r characters are added automatically by chinfusor, characters escaping is not supported.

Since version 1.2, Chinfusor tracks its documentation files, if they're present on its startup. Any changes made in them will be reflected immediately, without a need to restart the current session.

### Installing Chinfusor

Installation of Chinfusor consists of two parts, getting Chinfusor's binary and moving it to the necessary place.

There is a precompiled 64-bit binary, packaged with the chinfusor distribution. I am using it on my Ubuntu mate 20.04 machine and it works, but I have no idea about other distributions.

Thus, if you want to compile Chinfusor from source, in case you have Rust installed, all you need to do is:

* Open a terminal, and navigate to the src/sd_chinfusor folder.
* cargo build \--release -q
* After the compilation finishes, ideally with no output, navigate to the target/release directory and use chmod 755 sd_chinfusor command.

In case you don't have Rust installed, I recommend reading [Rust's installation page,](https://www.rust-lang.org/tools/install) which contains everything necessary to get you going.

After you have optained your binary, you need to copy it to the folder, where speech-dispatcher stores its speech modules. On my computer, this is /usr/lib/speech-dispatcher-modules/, but it can be different on your machine, so make sure to check out, that you're copying into right location. The directory should contain executables starting with sd_, such as sd_espeak-ng, sd_espeak etc.
When you're sure about the target location, open a terminal, activate root mode with sudo -i, navigate to the folder with your sd_chinfusor binary and enter command:

cp sd_chinfusor /usr/lib/speech-dispatcher-modules/sd_chinfusor\

Don't forget to log out from root mode by exit command.

Now, log out and log in again in order for your changes to take effect. Open Orca's settings by pressing orca+space, navigate to the Speech or voice tab (I'm not sure about the exact English version, it's called Hlas on my machine) and check out the list of available synthetizers. If you see Chinfusor there, then congrats, you've set up your Chinfusor successfully!

However, before confirming it as your default synthetizer, I recommend reading the next section.

### Ensuring, that Chinfusor works correctly

If you've ever created and published your own application, you most likely know very well, that there's a big difference between a technically working app, and an app ready to face any user. People are naturally good at finding the every possible way to break something, so programmers need to invest lot of efforts just into making their apps foolproof.

Chinfusor is no exception in this rule. I've created it as a project for my personal use, as I wanted to learn Chinese, and there were no suitable ways on Linux to do it for a blind person. I know every comma of the app's source code, and I know how to work with it without breaking anything.\
Because the idea is mostly general and could help other people as well, I've decided to publish it, so everyone can read texts in foreign alphabets naturally on Linux. However, making it completely foolproof would be another story, which I don't have a time nor motivation to deal with.\
However, because as I said before for blind users, speech is absolutely crucial to work with computer, I've made few steps to help you detect problems before you switch to Chinfusor as your primary speech engine.

After you install Chinfusor and make sure, that it's visible in Orca's speech synthesizer selection combo-box, launch the speech-dispatcher-cli application, packaged with Chinfusor distribution. The binary is again for Ubuntu mate 20.04 64-bit, and you can compile your own version by navigating to src/speech-dispatcher-cli in terminal and entering:

cargo build \--release -q

Note that before building, you may need to install speech-dispatcher development package:

sudo apt install libspeechd-dev

Thanks to Nolan Darilek for creating a great Rust wrapper around this package, his work saved me a lot of time during creation of my first applications on Linux platform.

speech-dispatcher-cli is a very simple terminal interface to speech-dispatcher, developed specially to test Chinfusor. After launching, every text entered is spoken and every input containing = character evaluated as a command. You can set module, pitch, rate and other parameters of synthesis in this way.
After starting speech-dispatcher-cli, which by the way from the rust project's root can be also done by:

cargo run \--release -q

Switch to Chinfusor module:

module=chinfusor

(no output is okay) and enter a text containing all supported alphabets, for example:

hello, 你好, привет

You should hear hello, nihao and privet. If you don't, try classic espeak-ng:

module=espeak-ng

hello

And If you hear hello here, something is most likely wrong with your Chinfusor installation or configuration.

### What to do if my speech in Orca is suddenly frozen while using chinfusor?

Frozen speech is a nightmare of every blind computer guy, wether he / she is just a basic user or an experienced programmer. In current state of development, Chinfusor has a quite good implementation of speech-dispatcher's communication protocol, and shouldn't cause problems once you get it running.\
The most risky part is while installing or updating it, if you don't handle the configuration correctly, there could be some issues with proper functioning of the program. But even in this case, there are various barriers made to prevent Orca from being speechless. In the worst case, if you configure Chinfusor with invalid speech modules paths, it should crash and let itself be replaced with other engine, such as espeak.

However, if despite these protection measures you still stay without speech after selecting Chinfusor as your default engine, for example because you set the volume in configuration to -100 somehow and didn't check it with speech-dispatcher-cli, don't panic. There is still a way out of it without sighted assistance.\
First of all, try to open terminal. On ubuntu version 20.04, you can do so by pressing shortcut super+T, earlier versions of Ubuntu used ctrl+alt+T. Ensure that you're in the terminal window by pressing left and right arrow, you should hear the beep sound on both sides, as nothing is written there yet.\
Next, delete Chinfusor by typing:

sudo rm /usr/lib/speech-dispatcher-modules/sd_chinfusor

If you have feeling, that you made a mistake while typing, delete the whole text and type it again. Mistakes are not something you'd like on this place. After confirming the command, you will be asked for your root password. Type it and submit, you can verify, that it was accepted by pressing backspace, the password request doesn't produce the beeping sound.

Next, if you don't have any unsaved documents opened in the current session, reboot your system by command:

reboot

After restarting, Orca should start with espeak and you should be able to repair your Chinfusor configuration. 

If this way seems unreliable to you, I can confirm that it really works. Or at least, it did on my machine, I got stuck few times during the development, and I never needed sighted assistance to repair things.

### Sandboxing speech modules

If you think of it, speech modules are very sensitive programs for blind users. We have many security keys these days, such as passwords, but also credit-card numbers, or security codes offered for example by gmail to bypass two-factor authentication. It's quite safe to store these informations on computer, if you use proper encryption, but no matter how much you encrypt the data, when you actually need them, they will eventually end up in... your speech module. And besides that, speech modules are running for the whole session time, so they have lot of time to do all sorts of other bad things, such as collect your private documents, your e-mails, record your microphone, track your browser history, track your clipboard and send everything to remote servers. And i'm not even speaking about ransomware, or simply deleting your documents or music, just for fun. 

By installing third-party speech modules, which are not included in Linux checked repositories, you're facing all these risks. It doesn't matter if the app is open or closed-source, who would bother with checking thousands of lines of code necessary to do task as complex as speech synthesis? Someone would just say not to install things from untrusted sources, but then you need to give up on potentially benefitial choice.

So, isn't there a better way to solve this problem?

Yes, there is. And with Chinfusor, it's very easy. Chinfusor allows you to run speech modules in sandbox, using [Firejail.](https://firejail.wordpress.com/) Firejail is a sandboxing tool for Linux, offerring powerful kernel-based protection even to regular users through simple profiles, in which you can specify the whole running envyronment for any app on your computer.\
All you need to do in order to sandbox a speech module is to create a profile for it, in which you set the necessary restrictions. For example, why would a speech module need an Internet connection? Why would a speech module need to write anything to your drive, or be able to access your documents? You can simply prohibite all of these things. Without a working connection, any espionage activities are useless. Without writable access to important parts of your filesystem, any malware is practically harmless.\
After you set up your profile, and test it possibly with another program, you simply rewrite your Chinfusor configuration to sandbox particular module and with next login, you'll be safe.

Note that if you're already using other sandboxing technologies, you might want to check compatibility between them and Firejail before using it.\
Note2, before using firejail, it must be installed on your system.

sudo apt install firejail

You don't need it in case you won't use sandboxing.

### How to update Chinfusor

Because Chinfusor doesn't contain a self-updating mechanism, you might wonder, how to update it properly, when a new version comes out. My recommendation on this topic is as follows:

1. Download a new version of Chinfusor from its official page.
2. If the new version is just few releases ahead of your currentone, I recommend checking changelog to see what changed and making the necessary changes. If you have a very old version and you don't want to read through the whole log, I recommend reading Installation section again, where you can see, what is different from what you used to do in setup.
3. Switch your Orca synthesiser to anything else than Chinfusor, for example espeak-ng.
4. Install Chinfusor according to installation instructions.
5. After logging out and in, check your Chinfusor installation and configuration with speech-dispatcher-cli program.
6. When you're sure that everything works, switch orca back to Chinfusor.

## Internal structure of Chinfusor

This section is primarily aimed for developers, who might want to either play with or study Chinfusor's code. If you don't belong to this group, you can continue directly to the next section.

Because my code as usual doesn't contain a single line of comment, may be except for unused code parts, which I was lazy to delete, I want to at least briefly describe here the internal structure of the program. Chinfusor consists of two modules, chinfusor and text_processor. The former is responsible for running the program, the latter contains logic for text parsing.

After program's start, speech modules are loaded, handling of subprocess is done by Process structure. This structure contains stdin and a channel to thread asynchronously parsing stdout, sending lines as Strings to the receiver owned by instance of the Process structure. This approach was necessary, as it is not possible to determine, whether a pipe has new content without blocking the current thread. To prevent growing number of threads as more alphabets are added, a very simple ThreadPool is used. I have designed it specially for parsing output of speech modules and searching ending marks, but it could be in theory reused after modifying its Request enum. From outside, the Process structure provides synchronous method for writing to stdin and asynchronous method for reading lines from stdout, what is handy when waiting for particular engine to finish speaking and expecting stop requests at the same time.\
MiniThreadPool guarantees, that for whatever number of processes just one thread is used, but for a drawback, that reading from stdout of particular proces must be always activated manually with activate_async_reading_until_sd_end_signal method, which as name suggests, starts reading, which will progress untill an end mark is found.\ 
After speech modules are loaded, a new thread is started for parsing Chinfusor's stdin. This thread reads the standard input for speech-dispatcher's commands and processes them in appropriate way i.e. reads content and returns confirmations about receiving. When speech-dispatcher command is processed, it's turned into enum form and sent through channel to the main thread, which processes it further as necessary.\
back in the main thread, after starting the reading thread, a loop is started, which reads speech-dispatcher input channel either synchronously or asynchronously, depends on whether a speech is in progress.

## License

Chinfusor is an open-source project, licensed under the MIT license. In a short summary, the license states, that I, Rastislav Kiss, the project's author am not responsible for any damage you directly or indirectly cause by using this program, or in general anything you do with it. You can freely redistribute and modify it with mentioning the original author.

You can read the whole license text [here.](https://rastisoftslabs.com/wp-content/uploads/chinfusor/licence.txt) The same license is also packed with all Chinfusor releases, in included license.txt file.

By downloading and using this software, you agree to this license.

