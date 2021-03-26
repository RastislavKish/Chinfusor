# Changelog for Chinfusor

This is a changelog for Chinfusor and related programs.

## Version 1.1

* settings.csv is now called alphabets_settings.csv and instead of configuring alphabets, it defines them. Format is the same as in previous version, just with added unicode_ranges column, which is described in more detayl in documentation. User can define as many alphabets as he / she wishes, without any direct or indirect limitations.
* settings.conf, now contains general settings of Chinfusor.
* Reading output from speech modules is now handled by threadpool, so using more of them doesn't mean requesting more threads. Just one is required for this task, thus threads usage dropped from 5 in the default configuration to 3. This may seem like a small deal, but if user specifies 100 alphabets, the difference in required resources will be much more significant.
* Added some benchmarks to test out speed of text parsing. Current results on my laptop are 19 ms for one million latin characters and 33 ms for one million chinese characters. One million characters is about a size of a whole novel, so these results are very good. Test with 500000 latin and 500000 chinese characters mixed in groups of 10 took 39 milliseconds, what is again an encouraging result. All of these values show, that texts parsing is unnoticeable from the required time point of view.
* Added support for bopomofo characters to default alphabets definition.
* Code of the program was slightly reshaped to fit the new requirements. It consists now of just one crate - sd_chinfusor, containing all the code.
* Fixed two bugs related to processing KEY requests, which as I found were capable of freezing Chinfusor. It was an interesting finding, as I thought, that Orca uses these, but that's obviously not the case. Anyway, the function is now stabilized and safe.
* Updated documentation, especially the configuration section, a section about updating Chinfusor was added and few other places were edited to reflect current state of the program.

## 1.0

The first release.

