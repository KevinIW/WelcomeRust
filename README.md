<details>

<summary>Modul 6</summary>

# Commit 1 Reflection notes
Kode Rust tersebut merupakan implementasi dari sebuah server TCP sederhana yang pake listener saat let listener koneksi masuk pada IP 127.0.0.1:7878. Ia dapat menangani setiap koneksi dengan membaca permintaan HTTP per baris di function handle_request. Saat koneksi baru diterima, server membaca setiap baris permintaan HTTP (buffreader) secara berturut-turut hingga menemui baris kosong yang menandakan akhir dari request. Selanjutnya, server mencetak request HTTP tersebut ke komputer. Di kode tersebut saat di loop akan menghandle request. Jadi bisa beberapa request yang dijalankan menghasilkan hasil yang berbeda.


# Commit 2 Reflection notes

[image1](commit2.png)

Kode fn handle_connection(mut stream: TcpStream) 
menerima sebuah parameter stream yang merupakan sebuah TcpStream. TcpStream adalah tipe yang digunakan untuk membaca dan menulis data melalui koneksi TCP.

let buf_reader = BufReader::new(&mut stream);
Kode ini membuat sebuah BufReader yang digunakan untuk membaca data dari stream. BufReader digunakan untuk membaca data secara efisien dengan melakukan buffering

let http request
membaca baris-baris dari stream, mengumpulkannya ke dalam sebuah Vec<_>, dan kemudian menghentikan proses pembacaan ketika sebuah baris kosong ditemukan.

let status line mengirim respons berhasil jika berhasil

let content utk membaca file dr path yg dikasi

let length = contents.len();
Ini menghitung panjang dari isi file yang telah dibaca sebelumnya.

let response = format!("{status_line}\r\nContent-Length:{length}\r\n\r\n{contents}");
Kode ini membuat respons HTTP dengan menggunakan format string. Variabel status_line dan length akan dimasukkan ke dalam respons. Setelah header respons, baris kosong diperlukan untuk menandakan akhir dari header HTTP, kemudian diikuti dengan isi file HTML.

stream.write_all(response.as_bytes()).unwrap();
Kode ini menuliskan respons yang telah dibuat ke dalam stream. .as_bytes() digunakan untuk mengonversi respons dari tipe String menjadi slice byte. 

# Commit 3 Reflection Notes

[image2](commit3.png)

Pada kali ini jika mengakses link selain dari /hello maka akan menimbulkan bad request. Dengan melakukan refactoring hanya menambahkan bad.html pada templates dan mengubah request method. Kita harus menggunakan refactoring karena mempercepat pengerjaan dan menghemat waktu dari berbagai potensi bug. 

Pada kode let request method pertama memecah url sehingga mendapatkan requestnya serta pathnya. 

<br>let (request_method, path) = if let Some(first_line) = http_request.first() {
        let mut parts = first_line.split_whitespace();
        let method = parts.next();
        let path = parts.next().unwrap_or("/");
        (method, path)
    } else {
        (None, "/")
    };</br>

lalu pada bagian if tinggal sama kan dengan path /hello maka akan mengakses hello.html

<br> // Check if the request method is GET and if the path is "/hello"
    if request_method == Some("GET") && path == "/hello" {
        status_line = "HTTP/1.1 200 OK";
        contents = fs::read_to_string("src/templates/hello.html").unwrap
        });
    } else {
        status_line = "HTTP/1.1 400 Bad Request";
        contents = fs::read_to_string("src/templates/bad.html").unwrap_or_else(|_| {
            
</br>

# Commit 4 Reflection Notes

Ketika mengakses "/sleep", server akan mengalami penundaan selama 5 detik sebelum konten dari "/hello" . Penundaan ini disebabkan oleh penggunaan fungsi thread::sleep(Duration::from_secs(5)); dalam kode program. Fungsi ini memaksa thread eksekusi untuk diam selama periode waktu yang ditentukan sebelum melanjutkan eksekusi program. Oleh karena itu, ketika mengakses "/sleep", server akan menunda respon selama 5 detik sebelum dapat memberikan konten "/hello". Sementara itu, ketika mengakses "/hello", tidak ada perintah penundaan, sehingga server dapat langsung memberikan konten kepada pengguna tanpa ada penundaan tambahan. Ini menyebabkan perbedaan dalam kecepatan respons antara dua akses tersebut. Dengan demikian, penggunaan fungsi thread::sleep mengakibatkan penundaan dalam memberikan respons, sementara akses yang tidak melibatkan penundaan dapat memberikan respons secara langsung.


# Commit 5 Reflection Notes

ThreadPool multi-threaded memungkinkan eksekusi tugas-tugas secara paralel di beberapa thread, meningkatkan kinerja aplikasi dengan menggunakan sumber daya CPU yang tersedia secara lebih efisien. Dengan ThreadPool, aplikasi dapat tetap responsif terhadap peristiwa eksternal seperti interaksi pengguna atau permintaan jaringan tanpa menghalangi thread utama. Ini sangat bermanfaat dalam aplikasi yang membutuhkan pengolahan data besar, pemrosesan real-time, atau pengelolaan koneksi jaringan dalam lingkungan server. Penggunaan ThreadPool membantu mengoptimalkan penggunaan sumber daya, seperti CPU, memori, dan IO, dengan menangani tugas-tugas secara paralel tanpa perlu menunggu tugas sebelumnya selesai. Secara keseluruhan, ThreadPool memberikan cara yang efisien untuk meningkatkan responsivitas, kinerja, dan efisiensi aplikasi dalam pengolahan tugas-tugas secara bersamaan. Threadpool ini bisa dibuat dengan mudah menggunakan bahasa rust.











</details>