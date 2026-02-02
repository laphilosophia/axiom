# Document Orchestrator: Nedir, Ne Değildir?

Bu proje, yerel dosyalarınızı organize etmek ve yönetmek için tasarlanmış pratik bir araçtır. Karmaşık bilişim teorileri, "yapay zeka hafızası" ya da "bilgi grafikleri" ile uğraşmaz. Hedefimiz basitlik ve netliktir.

### Temel Sınırlarımız

Bu sistem, dosya sisteminiz üzerinde çalışan hafif bir web arayüzüdür. Bir dokümanın oluşturulmasından arşivlenmesine kadar geçen süreci yönetir. Bir "öğrenen sistem" veya kurumsal bir motor değildir; sadece doküman hijyeni sağlar.

---

## Çözmek İstediğimiz Sorun

Dosya sayısı arttıkça şu problemler baş gösterir:

* Bazı dokümanlar güncelliğini yitirir ama orada durmaya devam eder.
* Birbiriyle ilgili dosyalar kopuk kalır.
* Daha önce yazılmış bir şey unutulur ve sıfırdan tekrar yazılır (duplikasyon).

Düz arama motorları bu dağınıklığı önlemeye yetmez. Bu araç, dokümanlarınızı **kaybolmaktan** ve **tekrar tekrar yazılmaktan** kurtarır.

---

## Sistemin İşleyişi

Sistem, dosyalarınızı yerel sürücüde saklar ve onlara birer "durum" etiketi atar. Temel amacı, size "Bu doküman hala geçerli mi?" ve "Buna benzer bir şey zaten var mı?" sorularının cevabını vermektir.

### Doküman Yapısı

Her doküman iki parçadan oluşur:

1. **İçerik:** Markdown veya düz metin (Sistem içeriğe müdahale etmez).
2. **Kimlik Kartı (Metadata):** Başlık, oluşturulma tarihi, etiketler ve güncel durum (taslak, aktif, arşivlenmiş gibi).

### Yaşam Döngüsü

Dokümanlar statik değildir; bir süreçten geçerler:

* Yeni başlanan bir iş **taslaktır**.
* Tamamlanınca **aktif** olur.
* Üzerine yeni bir versiyon yazılırsa **eskimiş (superseded)** olarak işaretlenir.
* Artık ihtiyaç duyulmayanlar ise **arşivlenir**.

---

## İlişkiler ve Bağlantılar

Sistem, dokümanlar arasındaki bağları iki şekilde kurar:

1. **Manuel Bağlantılar:** Sizin tarafınızdan açıkça belirtilen ilişkilerdir. Örneğin; "A belgesi, B belgesinin yeni versiyonudur."
2. **Akıllı Öneriler:** Sistem; başlık benzerliği, etiket çakışması veya LLM (Yapay Zeka) desteğiyle size "Bak, elinde buna benzer bir doküman zaten var" diyebilir. Bu öneriler siz onaylamadığınız sürece kaydedilmez.

---

## Teknik Yaklaşım ve Arama

* **Veri Kaynağı:** Tek gerçek kaynak sizin dosya sisteminizdir.
* **Arama:** Dosya içeriğinde tam metin arama yapabilir, etiketlere veya doküman durumuna göre filtreleme yapabilirsiniz.
* **Yapay Zeka:** LLM kullanımı tamamen opsiyoneldir. Sadece "benzer doküman bulma" aşamasında tavsiye vermek için kullanılır; asla dosyanızın durumunu kendi başına değiştirmez.

**Başarı Kriterimiz:** Eğer kullanıcı, elinde zaten olan bir şeyi tekrar yazmayı bırakmışsa ve aradığı güncel dosyaya hızla ulaşabiliyorsa bu sistem görevini yapmış demektir.
