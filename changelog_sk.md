# Changelog Chinfusor

Toto je changelog zachytávajúci zmeny v Chinfusore a súvisiacich programoch.

## Chinfusor 1.2

* Bolo pridané sledovanie konfiguračných súborov, ak existujú. Akékoľvek zmeny v konfigurácii sa premietnu okamžite, bez nutnosti reštartovať session.
* Opravený bug v parsovaní ssml tagov, ktorý spôsoboval čítanie zahraničných textov oddelených medzerami po slovách.
* Aktualizovaná dokumentácia.

## Chinfusor 1.1

* settings.csv sa ponovom volá alphabets_settings.csv, a namiesto konfigurácie abecied ich definuje. Formát je rovnaký ako v predchádzajúcej verzii, pridaný bol iba stĺpec unicode_ranges špecifikujúci rozsah danej abecedy viz. dokumentácia. Používateľ/ka môže definovať ľubovoľný počet abecied bez akýchkoľvek priamych či nepriamych limitácií.
* Bol pridaný konfiguračný súbor settings.conf, ktorý obsahuje všeobecné nastavenia Chinfusoru.
* Čítanie výstupu z rečových modulov ponovom spravuje threadpool, vďaka čomu nie je potrebné nové vlákno pre každú abecedu. Ľubovoľné množstvo modulov si vystačí s jedným vláknom, vďaka čomu kleslo použitie vlákien z 5 v predvolenej konfigurácii na 3. Môže sa to zdať ako malá vec, no pri 100 definovaných abecedách by bol rozdiel vo vyžadovaných prostriedkoch oveľa výraznejší.
* Bolo pridaných pár benchmarkov, aby sa otestovala rýchlosť parsovacieho procesu. Na mojom laptope bolo 1000000 latinských znakov rozparsovaných zhruba za 19 milisekúnd, 1000000 čínskych znakov zabralo spracovať približne 33 milisekúnd. Mix latinských a čínskych znakov v skupinkách po desiatich trval spracovať 39 milisekúnd. To sú veľmi dobré výsledky, milión znakov je zhruba dĺžka románu, čo ukazuje, že táto operácia je časovo prakticky nepostrehnuteľná.
* Pridaná podpora pre bopomofo znaky v predvolenej definícii abecied.
* Kód programu bol mierne reštrukturovaný aby lepšie odrážal nové požiadavky. Už sa nejedná o workspace, tvorí ho iba jedna crate sd_chinfusor, ktorá zlučuje kód z oboch predchádzajúcich.
* Opravené dva bugy týkajúce sa spracovávania KEY požiadaviek, ktoré spôsobovali mrznutie Chinfusoru. Ich existencia ma celkom prekvapila, nakoľko som mal za to, že Orca tieto requesty využíva, no to zjavne neplatí. V každom prípade boli chyby odstránené, funkcionalita stabilizovaná a je ponovom bezpečná na používanie.
* Aktualizovaná dokumentácia, hlavne sekcia týkajúca sa konfigurácie, bola pridaná sekcia o aktualizácii Chinfusoru. Pár ďalších miest bolo upravených tak, aby odrážali aktuálny stav programu.

## Chinfusor 1.0

Prvé vydanie.
