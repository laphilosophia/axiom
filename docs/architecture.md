## Mimari Tasarım Belgesi: Yerel Doküman Orkestratörü

Bu belge; düşük kaynak tüketimi, yüksek performanslı yerel arama ve anlamsal ilişkilendirme odaklı sistem mimarisini tanımlar. Sistem, doküman hijyenini sağlamak amacıyla deterministik arama ile olasılıksal (LLM tabanlı) önerileri birbirinden ayırır.

---

### 1. Sistem Mimarisi Genel Bakış

Sistem, hibrit bir yerel uygulama yapısına sahiptir. Kullanıcı arayüzü hafif bir web katmanı (Preact) üzerinden sunulurken, ağır hesaplama ve veri yönetimi işleri Rust çekirdeği (Tauri Core) tarafından yürütülür.

| Katman | Teknoloji | Görevi |
| --- | --- | --- |
| **Frontend** | Preact + TypeScript | UI render, durum yönetimi, dosya önizleme. |
| **Host Bridge** | Tauri (Rust) | OS erişimi, IPC yönetimi, güvenlik kapsamları (scoping). |
| **Metadata Store** | SurrealDB (Embedded) | İlişkisel ve grafik verisi (supersedes, references). |
| **Search Engine** | Tantivy | Dosya içeriği üzerinde tam metin arama. |
| **Inference Engine** | ONNX Runtime (ort) | Vektör embedding üretimi (Paraphrase-multilingual). |

---

### 2. Veri Yönetimi ve Depolama

Sistemde veriler üç farklı formda tutulur:

#### 2.1. Kaynak Dosyalar (Filesystem)

Dosya sistemi yegane gerçeklik kaynağıdır (Single Source of Truth). Dokümanlar kullanıcı tarafından belirlenen klasörlerde Markdown veya düz metin olarak saklanır.

#### 2.2. Metadata ve İlişkiler (SurrealDB)

Dokümanların yaşam döngüsü ve birbirleriyle olan bağları SurrealDB üzerinde tutulur.

* **Graph Özelliği:** `supersedes` ve `references` bağları grafik kenarları (edges) olarak modellenir. Bu, bir dökümanın tüm versiyon geçmişini (lineage) tek bir sorguyla çekmeyi sağlar.
* **Tablo Yapısı:** `documents` (id, title, status, path, created_at, updated_at).

#### 2.3. İndeksleme (Tantivy)

Dosya içerikleri Tantivy ile diske indekslenir. Bu sayede binlerce döküman arasından anlık (sub-millisecond) anahtar kelime araması yapılır.

---

### 3. Anlamsal Analiz Katmanı (Semantic Layer)

Dökümanlar arasındaki benzerlikleri tespit etmek ve duplikasyonu önlemek için **ONNX Runtime** üzerinden yerel embedding modeli çalıştırılır.

* **Model:** `Paraphrase-multilingual-MiniLM-L12-v2`.
* **İşlem:** Döküman kaydedildiğinde veya güncellendiğinde, metin 384 boyutlu bir vektöre (embedding) dönüştürülür.
* **Benzerlik Hesabı:** SurrealDB üzerinde saklanan vektörler arasında **Kosinüs Benzerliği** (Cosine Similarity) hesaplanarak eşik değerin üzerindeki dosyalar kullanıcıya "öneri" olarak sunulur.

---

### 4. İş Akışı (Data Flow)

1. **Giriş:** Kullanıcı yeni bir döküman kaydeder.
2. **Olay (Event):** Tauri Rust tarafında bir `fs::write` işlemi tetiklenir.
3. **İşleme Hattı (Pipeline):**

* **Tantivy:** Dosya içeriği full-text indeksine eklenir.
* **ONNX:** Metin embedding modelinden geçirilir ve vektörü çıkarılır.
* **SurrealDB:** Metadata ve vektör verisi kaydedilir; varsa `supersedes` ilişkisi kurulur.

1. **Geri Bildirim:** Kullanıcıya, veritabanındaki mevcut benzer dökümanlar bir liste halinde sunulur.

---

### 5. Kritik Teknik Tercihler ve Gerekçeleri

* **Preact Seçimi:** React ekosisteminin gücünü korurken, bundle boyutunu ve bellek kullanımını minimize etmek.
* **SurrealDB (Embedded) Seçimi:** SQLite'ın ilişkisel gücü ile Graph veritabanlarının bağlantı kolaylığını tek bir Rust kütüphanesinde birleştirmek.
* **ONNX Runtime Seçimi:** Python bağımlılığı olmadan, `ort` crate'i ile modelin CPU üzerinde en yüksek performansla çalışmasını sağlamak.
* **Multilingual Model:** Dokümanların Türkçe ve İngilizce karışık olması durumunda dahi anlamsal bağları korumak.

---

### 6. Başarı Kriterleri ve Sınırlar

* **Sınır:** Sistem asla döküman içeriğini kendiliğinden değiştirmez; sadece metadata ve ilişkileri yönetir.
* **Performans:** 10,000 dökümana kadar arama ve benzerlik sorguları <100ms sürmelidir.
* **Güvenlik:** Tüm işlemler yereldir; hiçbir veri dışarıdaki bir servise (Cloud LLM vb.) aktarılmaz.
