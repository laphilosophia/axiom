### 1. Metadata Portability (Veri Kilidi Sorunu)

Sistemde SurrealDB ve Tantivy ikincil veri depolarıdır; asıl gerçeklik dosya sistemidir. Ancak, SurrealDB çökerse veya projeyi başka bir bilgisayara taşırsan tüm "supersedes" ilişkilerini ve durum bilgilerini (active/archived) kaybedersin.

* **Öneri:** Her dökümanın yanına (veya merkezi bir gizli klasöre) küçük bir `.sidecar.json` dosyası yazdır.
* **Faydası:** SurrealDB sadece bir "hızlı erişim indeksi" (cache) haline gelir. Veritabanını silsen bile, sistem döküman klasörünü tekrar taradığında tüm ilişkileri (lineage) sidecar dosyalarından geri yükleyebilir.

### 2. "Lineage" Görselleştirmesi

Dökümanlar birbirini eskittiğinde (supersedes), ortaya doğrusal olmayan bir tarihçe çıkar. Sanat okulu geçmişine atıfla; bu zinciri sadece bir liste olarak değil, döküman kartının kenarında ince, dikey bir "zaman çizgisi" veya "soy ağacı" izi olarak göstermelisin.

* **Detay:** Kullanıcı dökümana bakarken, bu dökümanın hangi dökümandan türediğini ve hangisi tarafından eskitildiğini tek bakışta, minimal bir grafik bağlamında görmeli.

### 3. "Cold Start" ve İlk Tarama

Eğer sisteme 20 yıllık döküman arşivini bir anda verirsen, ONNX embedding süreci saatlerce sürebilir ve sistemi kitleyebilir.

* **Strateji:** İlk taramada (Initial Scan) sadece Tantivy indeksi oluştur (Full-text search saniyeler sürer). Anlamsal embedding (ONNX) işlemini arka planda, dökümanlar dökümanlar "açıldıkça" veya sistem boşta (idle) kaldıkça parça parça yap.

---

### Teknik Özet Tablosu

| Bileşen | Kritik Risk | Çözüm Yaklaşımı |
| --- | --- | --- |
| **SurrealDB** | DB Bozulması / Taşıma Zorluğu | JSON Sidecar sync mekanizması. |
| **ONNX Engine** | Yüksek CPU / İlk Tarama Yükü | Lazy-loading embedding (Sadece ihtiyaç anında). |
| **Tantivy** | Index Corrupt (Beklenmedik kapanma) | `IndexWriter` safe-shutdown ve otomatik rebuild. |
| **UI (Preact)** | Büyük Liste Performansı | Windowing (Virtual Scroll) ve Signal tabanlı metadata update. |

---

### Taslak Şema: "Supersedes" İlişkisi (SurrealDB)

Bu yapı, bir dökümanın geçmişini ve geleceğini tek bir graph sorgusuyla çekmeni sağlar:

```surrealql
-- Döküman kaydı
DEFINE TABLE documents SCHEMAFULL;
DEFINE FIELD status ON documents TYPE string ASSERT $value INSIDE ["draft", "active", "superseded", "archived"];

-- İlişki (Edge) tanımı
DEFINE TABLE supersedes SCHEMAFULL;
DEFINE FIELD in ON supersedes TYPE record(documents);
DEFINE FIELD out ON supersedes TYPE record(documents);

-- Sorgu: Bir dökümanın tüm geçmişini getir
SELECT ->supersedes->documents AS history FROM documents:user_doc_id;
```
