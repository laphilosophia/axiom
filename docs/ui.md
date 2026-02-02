## 1. Görsel Konsept: "The Monolith"

* **Tema:** Derin Charcoal (#121212) arka plan, Akrilik/Cam efektleri (Glassmorphism) ve vurgu rengi olarak "Electric Indigo".
* **Tipografi:** UI için `Inter`, teknik veriler ve metadata ID'leri için `JetBrains Mono`.
* **Dokunma Hissi:** Hover durumlarında hafif "glow" efektleri ve Preact tarafında akıcı (Spring-based) geçişler.

---

## 2. Arayüz Detayları

### A. Sol Panel (Library & Workspace)

Sadece bir liste değil, dökümanların "enerjisini" gösteren bir alan.

* **Status Indicators:** Durumlar sadece metin değil, renkli minik neon noktalar:
* `Active`: Parlak Yeşil (Pulse efekti).
* `Superseded`: Sönük Kehribar (Üzeri çizili başlık).
* `Draft`: Kesikli beyaz çerçeve.

* **Glassmorphism:** Panel arkası hafif bulanık (Blur: 10px), ana içerikten bir katman yukarıda hissettirir.

### B. Orta Panel (The Infinite Feed)

Arama sonuçları ve döküman kartları burada akar.

* **Card Design:** Her döküman bir "kart" değil, yatay birer "şerit" gibi görünür.
* **Micro-interactions:** Bir dökümanın üzerine gelindiğinde, sağ tarafta "Hızlı Aksiyonlar" (Arşivle, İlişkilendir) floating olarak belirir.
* **Search Highlight:** Tantivy'den gelen sonuçlarda, aranan kelimeler arkası parlayan bir sarı ile değil, altı çizili ve bold olarak vurgulanır.

### C. Sağ Panel (Orchestration Deck)

Burası senin "kontrol kulen".

* **Semantic Graph View:** SurrealDB'den gelen ilişkileri (Supersedes/References) minik birer node yapısı olarak gösteren, interaktif bir mini-harita.
* **Similarity Toast:** Benzer bir döküman bulduğunda, ekranın sağ üstünde değil, editörün hemen yanında "Anlamsal Çakışma %85" gibi şık, yarı şeffaf bir uyarı balonu.

---

## 3. Teknik UI Stack (Modern & Sexy)

| Parça | Tercih | Neden? |
| --- | --- | --- |
| **Styling** | **Tailwind CSS + Radix UI** | Radix, erişilebilir ve özelleştirilebilir "headless" bileşenler sunar. Muhasebe programı hissini bunlar yıkar. |
| **Icons** | **Lucide-React** | İnce çizgili, modern ve tutarlı ikon seti. |
| **Editor** | **Milkdown** | Markdown tabanlı, WYSIWYG ama "developer-friendly". |
| **Transitions** | **Motion (Framer Motion)** | Preact ile döküman durumları değişirken (örn: Draft -> Active) arayüzün "nefes almasını" sağlar. |

---

## 4. UX Magic

1. **Read-only Mode (Superseded):** Bir döküman eskidiğinde (superseded), editörün üzerine hafif bir "sepya" filtre iner ve sağ üstte "Bu döküman tarihe karıştı" minvalinde bir mühür görünür.
2. **Command Palette:** `Cmd+K` yaptığında açılan pencere sadece liste değil, bulanık bir overlay ile tüm ekranı kaplayan estetik bir "Spotlight" arayüzü olmalı.
3. **No-Save:** Her şey yerel (filesystem) olduğu için "Kaydet" butonu yerine, sağ altta minik bir "Disk synced" ikonu (Success check) göstererek güven tazele.
