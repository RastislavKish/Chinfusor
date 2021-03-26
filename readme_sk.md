# Chinfusor, univerzálne riešenie pre čítanie textov v cudzích abecedách pre Linux

Či už sa chcete naučiť cudzí jazyk, alebo ho dokonale ovládate, k základným zručnostiam patrí schopnosť čítať texty napísané v danom jazyku. V jazykoch s písmom založeným na latinskej abecede to s čítačom obrazovky nie je problém, texty možno čítať jednoducho tak, ako sú. Situácia je však podstatne komplikovanejšia pri písmach, ktoré majú s tými latinskými spoločné len máločo, presnejšie nič. Takými sú napríklad Čínska znaková abeceda, ruská azbuka či japonská kana.\
Keď vezmeme do úvahy, že populácia Číny v čase písania tohto textu činí zhruba 1,4 miliardy ľudí, približne každý piaty človek na svete hovorí po Čínsky. A napriek tomu, keď Espeak, predvolený hlas na väčšine prístupnýh linuxových distribúciach narazí na čínske znaky, jeho reakcia sa nesie v duchu (čajnýs znak, čajnýs znak, čajnýs znak, ...). Azbuka či kana nedávajú oveľa rozumnejší výstup.

Preto je tu Chinfusor, ktorý tento problém rieši.

## Ako to funguje

[Speech-dispatcher](https://freebsoft.org/speechd) je známym Linuxovským ekvivalentom na Windowse známeho sapi. Ide o službu, ktorá združuje rečové moduly ako Espeak či Festival, ktoré zas reprezentujú syntetické hlasy. Keď potom Orca, predvolený čítač obrazovky pre Gnome potrebuje vysloviť nejaký text, obráti sa na Speech dispatcher, ktorý požiadavku prepošle rečovému modulu, ktorý sa postará o syntézu.

Chinfusor je tiež takýmto rečovým modulom. Neobsahuje však syntézu sám o sebe, no interne načíta ďalšie moduly, a keď dostane text na vyslovenie, rozdelí ho na časti napísané v rôznych abecedách, pričom každú pošle do správneho enginu v správnom čase podľa konfigurácie.\
Tento prístup umožňuje nie len korektné čítanie textov napísaných v rôznych abecedách, no keďže všetko načítavanie a konfiguráciu rečových modulov riadi Chinfusor, môžete si pre každú abecedu nastaviť zvláštne parametre. Slovenčinu napríklad môžete chcieť čítať plným tempom, zatiaľ čo Ruštinu, ktorú sa možno len učíte uprednostníte čítanú pomalšie. Možno Vám vyhovuje robotický hlas Espeaku pre tuzemské jazyky, no pre tonálnu Čínštinu by ste radšej viac ľudsky znejúci prejav.\
Všetko toto je možné v Chinfusore nakonfigurovať, aby ste zo svojich hlasov mohli vyťažiť maximum.

A keďže je Chinfusor klasický rečový modul, môžete ho využívať neustále, priamo vo Vašom obľúbenom čítači obrazovky. Ak sa obávate, že bude dodatočné parsovanie textu a presmerovávanie požiadaviek cez procesy spôsobovať latenciu, vedzte, že Chinfusor je naprogramovaný v [Ruste,](https://rust-lang.org/), modernom programovacom jazyku, ktorý je nie len cool a veľmi bezpečný, no taktiež neskutočne rýchly, jeho hlavné ciele sú bezpečnosť, paralelnosť a rýchlosť. Prekladá sa priamo do strojového kódu, takže by ste nemali pociťovať nijaké spomalenie, ani pri tých najdivokejších situáciach.

## Ako to nainštalovať

Aby ste mohli chinfusor bezproblémovo používať, treba vykonať niekoľko úkonou.

### Konfigurácia

I keď sa Chinfusor bude držať predvolených nastavení ak nenájde súbory s konfiguráciou, je dobré ho nastaviť už pri prvej inštalácii, aby bolo možné jednoducho vykonávať zmeny. Skopírujte súbory alphabets_settings.csv a settings.conf do priečinka ~/.config/chinfusor, čo je konfiguračná lokalita Chinfusoru. Možno budete potrebovať povoliť zobrazovanie skrytých priečinkov a súborov vo Vašom správcovi súborov, nakoľko adresár .config je predvolene skrytý.

Súbor settings.csv je tzv. comma separated values konfigurácia, teda súbor s hodnotami oddelenými čiarkami. Obsahuje definície jednotlivých abecied a ich nastavenia. Jednotlivé hodnoty sa určujú v tomto poradí:

* Abeceda, abeceda, pre ktorú špecifikujete na danom riadku konfiguráciu. jej názov je čisto informatívny, Chinfusor s ním v zásade nijak nepracuje, preto si môžete zvoliť pomenovanie, aké sa Vám páči.
* Unicode rozsahy, rozsahy unicode pre danú abecedu. Formát je u0xa-u0xb, kde a a b sú začiatočný a konečný index v unicode tabuľke. 0x označuje hexadecimálnu hodnotu, bez neho sa čísla berú za decimálne. Možno zadať viacero rozsahov pre jednu abecedu naraz jednoduchým opakovaním tohto vzoru, pričom je možné použiť ľubovoľný oddelovač príp. žiadny. Hviezda (*) alebo prázdne miesto značí, že sa jedná o latinskú abecedu. V celej konfigurácii by mala byť špecifikovaná presne jedna latinská abeceda, ak je ich viac, berie sa do úvahy len prvá, ak menej, použije sa predvolená konfigurácia. Rozsahy sa nesmú vzájomne prekrývať, ináč je správanie neurčené.
* Modul, cesta k rečovému modulu, ktorý sa má pre danú abecedu použiť.
* Argument, argument pre uvedený rečový modul, typicky absolútna cesta k jeho konfiguračnému súboru.
* Jazyk, jazyk, ktorý sa má pre danú abecedu použiť, vo forme skratkového kódu napr. en, sk, ru atď.
* Hlas, hlas, ktorý má byť použitý, typicky male1, no rôzne syntetizéry môžu ponúkať rôzne hlasy.
* Miera interpunkcie, úroveň, s akou sa majú čítať interpunkčné znaky. Možné hodnoty sú none, some a all.
* Výška, výška, ktorá sa má pre danú abecedu použiť, od -100 po 100 vrátane.
* Výška veľkých písmen, výška, ktorou sa majú vyslovovať veľké písmená, od -100 po 100 vrátane.
* Rýchlosť, rýchlosť reči pre danú abecedu, od -100 po 100 vrátane.
* Hlasitosť, hlasitosť pre danú abecedu, od -100 po 100 vrátane.
* Firejail, špecifikuje, či sa má daný modul sandboxovať, yes a true túto možnosť zapínajú, všetko ostatné znamená vypnuté.

Poznámka, v prípade slovenských systémov môžete chcieť v pribalenej konfigurácii zmeniť jazyk latinského enginu z en na sk, aby Vám rozprával po Slovensky.

Poznámka 2, riadky začínajúce znakom # sa považujú za komentár.

Poznámka 3, hoci parsovanie konfigurácie obsahuje základné kontroly správnosti zadaných údajov, nie sú v žiadnom prípade pripravené na všetky situácie, a napríklad validita cesty k modulu sa vôbec nekontroluje. Odporúčam preto skontrolovať dva krát, čo do konfigurácie zadávate.

súbor settings.conf má jednoduchú štruktúru, prvý na riadku je vždy kľúč, potom dvojbodka s medzerou a hodnota. Zatiaľ je podporované iba nastavenie punctuation_characters, ktoré špecifikuje, ktoré znaky majú byť pri parsovaní považované za interpunkciu. # na začiatku riadku opäť označuje komentár.

### Inštalácia

Inštalácia Chinfusoru pozostáva z dvoch krokov, získania binárnej verzie a jej presunutia na správne miesto.

Chinfusor má pribalenú svoju 64-bitovú verziu skompilovanú pre Ubuntu mate 20.04. Na tomto systéme ju úspešne používam a nemám s ňou problémy, no ako sa bude správať na iných distribúciach netuším.

Môžete si skompilovať svoju vlastnú verziu z pribaleného zdrojového kódu, ak máte nainštalovaný Rust, postup je nasledovný:

* Otvorte terminál, a navigujte do priečinka so zdrojovým kódom (src/sd_chinfusor).
* cargo build \--release -q
* Po kompilácii, ktorá by v ideálnom prípade nemala nič vypísať do konzoly, navigujte do priečinka target/release a zadajte príkaz chmod 755 sd_chinfusor

Ak Rust nemáte, odporúčam prečítať si jeho [inštalačnú stránku,](https://www.rust-lang.org/tools/install) kde sa dozviete všetko potrebné.

Keď máte binárnu verziu enginu, musíte ju dostať na miesto, kde sú uložené moduly speech-dispatchera. Na mojom stroji je to /usr/lib/speech-dispatcher-modules, no u Vás môže byť cesta iná, preto odporúčam si ju najprv skontrolovať, cieľový priečinok by mal už obsahovať spustiteľné súbory začínajúce sa na sd_, ako sd_espeak-ng, sd_espeak a pod.

Keď ste si istý cestou, skopírujte Chinfusor na dané miesto ako root. Aktivujte si správcovské práva sudo -i, navigujte do priečinka so spustiteľným súborom a zadajte príkaz:

cp sd_chinfusor /usr/lib/speech-dispatcher-modules/sd_chinfusor\

Nezabudnite sa z rootu odhlásiť príkazom exit.

Následne sa odhláste a znova prihláste, aby ste začali nový session. Otvorte nastavenia Orci skratkou orca + medzerník a na karte hlas prezrite políčko Speech synthesizer. Ak medzi dostupnými možnosťami vidíte Chinfusor, tak gratulujem, práve ste nainštalovali rečový modul.

Nenastavujte ho však hneď ako predvolený, odporúčam prečítať si najprv nasledujúcu sekciu.

### Uistenie sa, že chinfusor funguje správne

Ak ste už niekedy naprogramovali a vydali nejaký program, tak určite viete, že je veľký rozdiel medzi technicky fungujúcou aplikáciou, a programom pripraveným zniesť akéhokoľvek používateľa. Ľudia sú veľmi dobrý v nachádzaní všetkých možných ciest, ako niečo pokaziť, preto programátori vkladajú množstvo úsilia len do toho, aby boli ich aplikácie blbuvzdorné, a nebolo ich také jednoduché rozhodiť.

Chinfusor z tohto pravidla nie je výnimkou. Jednalo sa o program, ktorý som vytvoril pre svoje súkromné použitie, nakoľko som sa chcel učiť po Čínsky a nebola možnosť, ako to na Linuxe rozumne spraviť. Poznám každú čiarku jeho kódu a viem, ako s ním pracovať tak, aby všetko fungovalo ako má. Pretože sa jedná o celkom všeobecný problém a riešenie by mohlo pomôcť viacerým ľuďom, rozhodol som sa ho zverejniť, nech z neho profitujú aj ďalší. Nemám však čas a pravdupovediac ani chuť vytvárať milión opatrení iba proti ľuďom, ktorí nečítajú dokumentáciu a niečo tak pokazia.

Keďže je však fungujúca reč kritická pre nevidiaceho používateľa počítača, urobil som aspoň niekoľko krokov pre možnosť včasného odhalenia problémov, aby ste sa mohli uistiť, že všetko funguje tak ako má ešte pred tým, než nastavíte Chinfusor ako svoj predvolený rečový modul.

Po nainštalovaní Chinfusoru a overení jeho prítomnosti v zozname rečových modulov spustite aplikáciu speech-dispatcher-cli, pribalenú k enginu. Predkompilovaná verzia je opäť pre Ubuntu mate 20.04 64-bit, môžete si tiež skompilovať svoju vlastnú.

najprv, ak ho ešte nemáte, nainštalujte si balíček libspeechd-dev z predvoleného repozitára vašej Linuxovej distribúcie:

sudo apt install libspeechd-dev

Vďaka Nolanovi Darilekovi, ktorý spravil pre tento balíček výborný Rust wrapper, čím mi ušetril kopu práce s mojimi prvými Linuxovými programami.

Speech-dispatcher-cli môžete z jeho priečinka (src/speech-dispatcher-cli) následne skompilovať príkazom:

cargo build \--release -q

a rovno aj spustiť:

cargo run \--release -q

speech-dispatcher-cli je jednoduché terminálové rozhranie k speech-dispatcheru. Čokoľvek zadané je vyslovené. Ak vstup obsahuje =, vyhodnotí sa ako príkaz, možno takto nastavovať rôzne parametre reči ako napríklad použitý modul, jazyk, výšku, rýchlosť a podobne.

Zvoľte Chinfusor ako použitý modul:

module=chinfusor

A zadajte na vyslovenie text obsahujúci časti vo všetkých podporovaných abecedách, napríklad:

ahoj, 你好, привет

Mali by ste počuť ahoj, nihao a privět. Ak nie a nedostali ste žiadny výstup, skúste sa prepnúť na espeak-ng:

module=espeak-ng

A zadať

ahoj

Ak budete počuť ahoj, tak je pravdepodobne niečo zle s Vašou Chinfusor inštaláciou alebo konfiguráciou.

### Čo robiť, ak sa náhle reč Orci zasekne počas používania Chinfusoru?

Nefungujúca reč je nočná mora každého nevidiaceho pracujúceho s počítačom, či už sa jedná o bežného používateľa, alebo skúseného programátora. Chinfusor má v súčasnosti celkom dobrú implementáciu protokolu používaného speech-dispatcherom a vstavané ochranné opatrenia by mali podobnému problému za každú cenu zabrániť. V najhoršom prípade, ak zadáte napríklad nefunkčnú cestu k rečovému modulu, engine crashne a nechá sa nahradiť iným modulom.

Stále však existujú spôsoby, ako sa k nemému stavu dopracovať, a to najme počas inštalácie či aktualizácie modulu. Ak po nastavení Chinfusoru ako predvoleného Orca syntetizéra zostanete bez reči, napríklad kvôli tomu, že ste nejakým zázrakom nastavili hlasitosť na -100, nemusíte panikáriť, dá sa z toho dostať aj bez zrakovej asistencie.

1. Otvorte terminál, na Ubuntu mate 20.04 skratkou super + T, na predchádzajúcich verziách ctrl+alt+T. Fakt, že ste v ňom môžete skontrolovať stláčaním šípky vľavo a vpravo, mali by ste v oboch smeroch počuť pípanie.
2. zadajte príkaz sudo rm /usr/lib/speech-dispatcher-modules/sd_chinfusor. Ak máte pocit, že ste pri písaní urobili chybu, odporúčam preventívne celý vstup zmazať a začať písať odznova, preklepy si tu veľmi nemôžete dovoliť.
3. Ak nemáte v aktuálnom sessione rozrobené neuložené dokumenty či inú prácu, zadajte príkaz reboot. Po reštarte systému by mala Orca naskočiť s Espeakom, a vy mať príležitosť opraviť inštalačné chyby.

Ak Vám tento spôsob príde nespoľahlivý či ťažkopádny, môžem potvrdiť, že reálne funguje. Aspoň teda na mojom stroji fungoval, Chinfusor sa mi pár krát zasekol počas vývoja, a nikdy som nepotreboval zrakovú asistenciu, aby som to  opravil.

### Sandboxovanie rečových modulov

Keď sa nad tým tak zamyslíte, rečové moduly sú pre nevidiaceho veľmi citlivé programy. V dnešnej dobe už máme množstvo rôznych bezpečnostných kľúčov, či už heslá, no tiež čísla kreditných kariet alebo kódy pre obídenie dvojfaktorovej autentifikácie, aké má napríklad Gmail. Mať ich uložené na počítači je v zásade v poriadku, ak ich správne zašifrujete, no bez ohľadu na to, aká ochrana je použitá, v našom prípade aj tak raz skončia... v rečovom module, keď ich po dešifrovaní budeme chcieť čítať.

Okrem toho, rečový modul beží po celú dobu trvania sessionu, počas ktorej má množstvo času na vykonávanie iných záškodníckych aktivít, ako nahrávanie zvuku z mikrofónu, sledovanie web-kamery, čítanie našich dokumentov, histórie prehliadania, e-mailov, sledovanie schránky, a odosielanie všetkých údajov na vzdialené servery. O ransomwaroch či jednoducho mazačských vírusoch, ktoré zmažú, čo im príde pod proces už ani nehovorím.

Podobné obavy sa netýkajú ani tak modulov typu Espeak, ktoré sú súčasťou kontrolovaných repozitárov, no skôr modulov tretích strán, ktorých pôvod a zámery nie vždy možno tak ľahko overiť. Môžu byť kľudne open-source, no komu sa chce kontrolovať tisíce riadkov kódu obsluhujúcich úkon tak zložitý, ako syntéza reči?

Niekto by jednoducho povedal, že netreba inštalovať softvér z neznámych zdrojov. Lenže, tým sa môžeme ukrátiť o potenciálne výhodnú voľbu. Neexistuje iné riešenie, ktoré by tento problém bezpečne vyriešilo?

Existuje, a s Chinfusorom je jeho aplikácia veľmi jednoduchá. [Firejail](https://firejail.wordpress.com/) je sandboxovací nástroj pre Linux, ktorý ponúka silnú ochranu založenú na bezpečnostných funkciách kernelu všetkým druhom používateľov cez jednoduché profily, ktorými môžu špecifikovať celé prostredie pre každú aplikáciu.

Po jeho nainštalovaní:

sudo apt install firejail

Vám stačí vytvoriť profil pre podozrivý rečový modul. V ňom uvediete všetky reštrikcie, ktoré chcete aplikovať. Načo potrebuje offline syntetizér prístup na internet? Načo je rečovému modulu prístup k Vašim dokumentom? A načo by vôbec mal mať oprávnenie zapisovať na disk? Všetky tieto veci môžete pomocou Firejailu jednoducho zakázať. Bez internetového pripojenia je akákoľvek špionážna aktivita zbytočná, lebo nemá ako poslať späť údaje. Bez možnosti zapisovať na disk je akýkoľvek malvér prakticky neškodný, lebo nemá moc napáchať akúkoľvek škodu.

Po vytvorení profilu stačí už len v konfigurácii Chinfusoru zapnúť sandboxovanie pre daný modul, a s jeho najbližším štartom budete v bezpečí.

Poznámka: Ak už používate iné sandboxovacie technológie, odporúčam overiť si kompatibilitu s Firejailom, hlavne formát sandbox v sandboxe býva problematický.\
Poznámka 2, Ak sandboxovanie nepotrebujete, Firejail mať nainštalovaný nemusíte. Používa sa iba vtedy, ak je tak špecifikované v konfigurácii.

### Ako aktualizovať Chinfusor

Keďže Chinfusor neobsahuje auto-aktualizačný mechanizmus, možno Vám napadla otázka, ako ho korektne aktualizovať, keď vyjde nová verzia. Osobne odporúčam nasledujúci postup:

1. Stiahnite si najnovšiu verziu Chinfusoru z oficiálnej stránky.
2. Ak je nová verzia len niekoľko čísiel popredu pred vašou aktuálnou, môžete si prečítať changelog a zistiť tak, čo sa líši a čo musíte urobiť, aby všetko správne fungovalo. Ak máte starú verziu programu a nechce sa Vám študovať všetky zmeny, odporúčam opätovne si prečítať sekciu inštalácie v dokumentácii, aby ste zistili, aký je aktuálny postup a ako sa líši od toho, čo ste zvykli robiť.
3. Prepnite aktívny syntetizér v Orce z Chinfusoru na hocičo iné, napríklad espeak-ng.
4. Nainštalujte Chinfusor.
5. Po odhlásení sa a opätovnom prihlásení otestujte program pomocou aplikácie speech-dispatcher-cli, aby ste sa uistili, že všetko funguje.
6. Prepnite váš aktívny rečový syntetizér v Orce opäť na Chinfusor.

## Interná štruktúra Chinfusoru

Táto sekcia je určená hlavne vývojárom, ktorý by chceli či už upravovať alebo študovať kód chinfusoru. Ak do tejto skupiny nepatríte, kľudne prejdite na ďalšiu sekciu.

Keďže môj kód ako zvyčajne neobsahuje ani riadok komentára, snáď s výnimkou starých častí kódu, ktoré sa mi nechcelo zmazať, chcem tu aspoň stručne popísať jeho fungovanie. Celý projekt sa skladá z dvoch modulov, chinfusor a text_processor. Prvá sa stará o beh programu samotného, druhá obsahuje logiku pre parsovanie textu.\
O správu rečových modulov sa stará štruktúra Process, ktorá nesie stdin a asynchrónne vlákno číta výstup po riadkoch z stdout, pričom po prečítaní jedného pošle tento cez kanál do inštancie štruktúry, ktorá ho vytvorila. Aby nepotreboval každý proces vlastné vlákno a neplitvalo sa prostriedkami, všetko ide cez jednoduchý ThreadPool, ktorý je navrhnutý špeciálne za účelom parsovania výstupu z rečových modulov. Navonok má programátor k dispozícii synchrónnu metódu write a asynchrónnu metódu read_line, ktorá vráti Option podľa toho, či je nový riadok k dispozícii alebo nie. To sa hodí najme v prípade, že treba súčasne čakať na správu z modulu o skončení syntézy a čítať príkazy speech dispatchera, keby prišiel pokyn na zastavenie.\
MiniThreadPool zabezpečuje, že ľubovoľný počet procesov bude potrebovať len jedno vlákno, no za cenu, že čítanie musí byť manuálne aktivované metódov activate_asynchronous_reading_until_sd_end_signal. Ako jej názov napovedá, táto metóda spustí čítanie, ktoré bude trvať až do zachytenia značky konca rozprávania.\
Po spustení programu sa najprv načítajú moduly, potom sa spustí vlákno čítajúce vstup programu. Toto vlákno komunikuje so speech-dispatcherom a premieňa jeho príkazy na varianty enumerátora, ktoré následne posiela cez kanál späť do hlavného vlákna.\
Tam medzi tým začne cyklus, ktorý zachytáva príkazy speech-dispatchera synchrónne alebo asynchrónne podľa toho, či sa práve hovorí alebo nie, a následne vstup matchuje, ak nejaký prišiel.

## Licencia

Chinfusor je open-source projekt, distribuovaný pod MIT licenciou. Tá v skratke hovorí o tom, že ja, Rastislav Kiss, ako autor tohto programu nenesiem nijakú zodpovednosť za akúkoľvek škodu priamo či nepriamo spôsobenú jeho používaním či vlastnením jeho kópie a celkovo za nič, čo s ním budete robiť. Máte právo slobodne ho kopírovať, upravovať či dokonca predávať za predpokladu, že uvediete autora pôvodnej verzie.

Celý text licencie, ktorá je taktiež pribalená k programu si môžete prečítať [tu.](https://rastisoftslabs.com/wp-content/uploads/chinfusor/licence.txt)

Sťahovaním a používaním tohto programu vyjadrujete súhlas s týmito licenčnými podmienkami.

