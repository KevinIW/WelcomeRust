<details>

<summary>Modul 6</summary>

# Commit 1 Reflection notes
Kode Rust tersebut merupakan implementasi dari sebuah server TCP sederhana yang pake listener saat let listener koneksi masuk pada IP 127.0.0.1:7878 dan menangani setiap koneksi dengan membaca permintaan HTTP per baris di function handle_request. Saat koneksi baru diterima, server membaca setiap baris permintaan HTTP (buffreader) secara berturut-turut hingga menemui baris kosong yang menandakan akhir dari request. Selanjutnya, server mencetak reqyest HTTP tersebut ke komputer.  


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






</details>