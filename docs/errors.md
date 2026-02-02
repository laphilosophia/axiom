## 1. Disk Dolu (Disk Full)

Tauri tarafında Rust kullanacağın için bu hata seni en çok `write` operasyonlarında vurur.

* **Atomic Writes:** Dosyayı doğrudan üzerine yazma. Önce bir `.tmp` dosyasına yaz, işlem başarılıysa `fs::rename` ile asıl dosyanın üzerine taşı. Eğer disk dolarsa `.tmp` yazımı hata verir, asıl dökümanın (Source of Truth) bozulmaz.
* **Metadata Tutarsızlığı:** Veritabanına (SurrealDB) "kaydedildi" yazıp disk yazımında hata alırsan ciddi sorun yaşarsın. Önce dosyayı diske yaz, sonra DB'yi güncelle.
* **UI Response:** Kullanıcıya "Yer kalmadı" demek yetmez; Tantivy indeksi için de yer gerektiğini hatırlatan bir "Sistem Kapasite Uyarısı" göstermelisin.

## 2. Dosya Yok Oldu (Missing File)

Kullanıcı senin arayüzün dışından (Explorer/Finder) dosyayı silebilir.

* **Ghost Entries:** Veritabanında döküman var ama diskte yok. Bu durumda UI dökümanı listeden kaldırmamalı; "Missing" (Kayıp) etiketiyle ve pasif (disabled) halde göstermeli.
* **Recovery:** Kullanıcıya iki seçenek sun: "Dökümanı sistemden (DB) temizle" veya "Dosyanın yeni yerini göster" (Relocate).
* **Lifecycle Impact:** Kayıp bir dökümanın durumu otomatik olarak "Archived" veya "Invalid" olarak işaretlenmelidir.

## 3. Bozuk Dosya (Corrupted File)

Döküman içeriği okunamaz hale gelmiş veya beklenmedik bir formatta olabilir.

* **Validation Gate:** Tantivy indeksleme ve ONNX embedding süreçleri başlamadan önce bir `UTF-8` kontrolü yap.
* **Opaque Handling:** Sistem dökümanı "opaque" (kapalı kutu) gördüğü için dökümanı açamasa bile metadata'yı (ID, etiketler) korumalıdır.
* **Flagging:** Dosya bozuksa, döküman kartına "Okuma Hatası: İndeksleme Yapılamadı" uyarısı basılmalı, ancak döküman yaşam döngüsünden çıkarılmamalıdır.

---

## 4. FS Watch mı, On-Demand mı?

Bu projenin kaderini belirleyecek teknik tercih burası.

### Hibrit Model (Önerilen)

Tamamen `watch` modunda kalmak, büyük döküman klasörlerinde Rust tarafında gereksiz CPU yükü ve "infinite loop" (uygulama yazar -> watch tetiklenir -> uygulama tekrar yazar) riski yaratır.

* **Watcher (Selective):** Sadece dosya **silinmelerini** ve **isim değişikliklerini** (rename) anlık takip et. Bu, "Ghost Entry" oluşmasını engeller.
* **On-Demand / Debounced (Content):** Kullanıcı dökümanı editörde değiştirirken her saniye embedding (ONNX) ve indeksleme (Tantivy) yapma. "Save" tetiğiyle veya editör boşta kaldığında (3-5 sn debounce) bu ağır işlemleri çalıştır.
* **Manual Re-Sync:** UI'a bir "Refresh Library" butonu koy. Bu, diskteki gerçek durum ile DB'yi zorla eşitleyen (Tantivy'yi baştan kuran) bir "acil durum" mekanizması olur.

---

### Hata Yönetimi Tablosu

| Senaryo | Rust Katmanı Aksiyonu | UI Katmanı Tepkisi |
| --- | --- | --- |
| **Write Fail** | `io::Error (StorageFull)` | Kırmızı toast mesajı + Yazma korumalı mod. |
| **Missing File** | `NotFound` (Surreal query mismatch) | Kart üzerinde "File Not Found" ikonu. |
| **Invalid UTF-8** | `from_utf8` error handling | "İçerik okunamıyor, sadece metadata düzenlenebilir." |
| **ONNX Crash** | Fallback to keyword search | "Anlamsal arama şu an devre dışı." |
