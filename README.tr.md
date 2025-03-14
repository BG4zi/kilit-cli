# kilit-cli

This is a cli based password manager written in Rust.
Rust dili ile yazılmış basit bir komut satırı arayüzü tabanlı şifre yöneticisi.



## Kullanımı
```bash
$ kilit-cli [SEÇENEKLER]
```
## OPTIONS
```txt
-c, --conf <conf>        Şifrelerin tutulduğu dosyanın dizinini belirler(dosyayla beraber)
                         	(Örnek kullanımı: "./kilit -c "$HOME/.mypassfile"") ve kullanıcı dizinin için ~ kısaltmasını kullanma, programı çökertiyor 
-h, --help               Seçenekleri yazdırır
-p, --prompt <prompt>    Yeni bir komut satırı arayüzü açmak yerine tek satırlık bir komutla işinizi görebilirsiniz                         
                             (Örnek kullanımı: kilit -c "~/.adana" -p "go passwd list name bgc"
                                               kilit -c "~/.adana" -p "create passwd") 
-V, --version            Versiyon bilgisini yazdırır
```
