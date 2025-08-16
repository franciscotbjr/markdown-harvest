## 1 - SAMPLE TEXT NO QUERY STRING:
```text
Rust is a general-purpose https://en.wikipedia.org/wiki/General-purpose_programming_language programming language https://en.wikipedia.org/wiki/Programming_language emphasizing performance https://en.wikipedia.org/wiki/Computer_performance, type safety https://en.wikipedia.org/wiki/Type_safety, and concurrency https://en.wikipedia.org/wiki/Concurrency_(computer_science). It enforces memory safety https://en.wikipedia.org/wiki/Memory_safety, meaning that all references point to valid memory. It does so without a conventional garbage collector; instead, memory safety errors and data races are prevented by the "borrow checker", which tracks the object lifetime of references at compile time.
```
### 1.1 - EXPECTED EXTRACTED URLS
   - https://en.wikipedia.org/wiki/General-purpose_programming_language
   - https://en.wikipedia.org/wiki/Programming_language
   - https://en.wikipedia.org/wiki/Computer_performance
   - https://en.wikipedia.org/wiki/Type_safety
   - https://en.wikipedia.org/wiki/Concurrency_(computer_science)
   - https://en.wikipedia.org/wiki/Memory_safety

## 2 - SAMPLE TEXT WITH QUERY STRING:
```text
Cavafy lived in England for much of his adolescence, and developed both a command of the English language and a preference for the writings of William Shakespeare http://www.poetryfoundation.org/archive/poet.html?id=6176 and Oscar Wilde http://www.poetryfoundation.org/archive/poet.html?id=7425. Cavafy’s older brothers mismanaged the family business in Liverpool, and Cavafy’s mother was ultimately compelled to move the family back to Alexandria, where they lived until 1882. Then Cavafy’s mother, sensing danger, returned to Constantinople with Cavafy and the rest of her children. When the British bombarded Alexandria, the Cavafy family home was destroyed in the battle, and all of Cavafy’s papers and books were lost.
```
### 2.1 - EXPECTED EXTRACTED URLS
   - http://www.poetryfoundation.org/archive/poet.html?id=6176
   - http://www.poetryfoundation.org/archive/poet.html?id=7425

## 3 - ONE MORE SAMPLE TEXT NO QUERY STRING:
```text
A language empowering everyone https://www.rust-lang.org/ to build reliable and efficient software. 
```
### 3.1 - EXPECTED EXTRACTED URLS
   - https://www.rust-lang.org/
