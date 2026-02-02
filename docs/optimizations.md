## 1. Watcher: "Sadece Kapıyı Dinle, İçeri Girme"

`notify` crate'i OS-level event'leri (inotify, fsevents) dinler, dolayısıyla boşta dururken CPU harcamaz.

* **Hata:** Her `Write` event'inde tüm boru hattını (indexing + embedding) tetiklemek.
* **Çözüm:** **Debouncing.** Kullanıcı dosyayı kaydettiğinde veya dışarıdan bir dosya geldiğinde 2-3 saniye bekle. Eğer bu süre içinde yeni bir event gelmezse işlemi başlat.
* **Filtreleme:** Sadece `.md` veya `.txt` uzantılarını dinle; geçici dosyaları veya `.git` klasörünü `ignore` et.

## 2. Tantivy: "Toplu Taşıma Kullan"

Tantivy'de her döküman için `commit` çağırmak IO ve CPU maliyetlidir.

* **Strateji:** Bellekte bir `IndexWriter` tut ve dökümanları `add_document` ile sıraya al.
* **Maliyet:** Asıl yük "segment merging" aşamasında biner. Bu işlemi sadece uygulama boştayken (idle) veya kapatılırken tetikle.
* **Deterministic Search:** Arama işlemi (query) zaten çok ucuzdur; Tantivy bu konuda Rust dünyasının en hızlısıdır.

## 3. ONNX (Embedding): "Asıl Canavar Bu"

`Paraphrase-multilingual-MiniLM-L12-v2` küçük olsa da CPU üzerinde ciddi bir matematiksel operasyon döndürür.

* **Queue (Kuyruk) Yapısı:** Yeni bir döküman geldiğinde bunu bir `Job Queue`'ya at. UI asla bu işlemin bitmesini beklememeli.
* **Hashing:** Bir dosya değiştiğinde önce içeriğin `SHA-256` hash'ini al. Eğer içerik anlamsal olarak değişmediyse (örneğin sadece bir boşluk eklendiyse), embedding modelini çalıştırma; eski vektörü kullan.
* **Background Priority:** Rust tarafında bu işlemi `thread::spawn` ile düşük öncelikli (nice) bir thread'de çalıştır. UI (Tauri/Preact) bu sırada 60 FPS akmaya devam etsin.

---

## Verimli İşleme Hattı (The Pipeline)

Sistemin CPU canavarına dönüşmemesi için izleyeceği yol şudur:

1. **Event:** Dosya değişti.
2. **Debounce:** 2 saniye sessizlik bekle.
3. **Hash Check:** Dosya gerçekten değişti mi? Hayırsa bitir.
4. **Tantivy:** Kelimeleri indeksle (Hızlı, <10ms).
5. **ONNX:** Eğer döküman "Active" durumuna geçtiyse embedding üret (Pahalı, 100-500ms).
6. **SurrealDB:** Metadata ve vektörü güncelle.

## 4. Akıllı Sınırlar (Hard Boundaries)

* **Indexing Scope:** Sadece `active` ve `draft` dökümanları anlık indeksle. `Archived` veya `Superseded` olanları sadece bir kez (ilk seferde) indeksle ve bir daha dokunma.
* **On-Demand Intelligence:** "Benzer dökümanları bul" (Similarity check) işlemini döküman her açıldığında değil, sadece kullanıcı "Analiz Et" dediğinde veya döküman durum değiştirdiğinde yap.

---

**Özet:** Boşta dururken CPU kullanımın **%0-1** bandında olmalı. Bir dökümanı işlerken kısa bir süreliğine (birkaç yüz milisaniye) bir "spike" (sıçrama) göreceksin, o kadar. Bu spike'ı da kullanıcıya hissettirmemek senin Preact tarafındaki asenkron yönetimine bakıyor.
