// 09-Funciones.
// =============================================================
// VER: https://doc.rust-lang.org/stable/rust-by-example/fn.html

// Una función que devuelve un valor booleano:
pub fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    // Caso extremo: división por `0`:
    if rhs == 0 {
        return false;
    }

    // Al ser esto una expresión, no es necesario `return`
    lhs % rhs == 0
}

// Las funciones que "no" devuelven un valor, en realidad devuelven el tipo de unidad `()`
pub fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

// Cuando una función devuelve `()`, el tipo de retorno puede
// omitirse en la firma de la función:
pub fn fizzbuzz_to(n: u32) {
    for n in 1..=n {
        fizzbuzz(n);
    }
}

// Funciones y Métodos asociados
// VER: https://doc.rust-lang.org/stable/rust-by-example/fn/methods.html

// Algunas funciones están conectadas a un tipo particular. Estos vienen en
// dos formas: funciones asociadas y métodos. Las funciones asociadas son funciones
// que generalmente se definen en un tipo, mientras que los métodos son funciones
// asociadas que se llaman en una instancia particular de un tipo.
// (NOTA: las funciones asociadas equivaldrían en OOP a los métodos estáticos.)

pub struct Point {
    pub x: f64,
    pub y: f64,
}

// Bloque de implementación, todas las funciones y métodos asociados a `Point` van aquí.
impl Point {
    // Esta es una "función asociada" porque esta función está asociada con
    // un tipo particular, es decir, Point.
    //
    // No es necesario llamar a las funciones asociadas con una instancia.
    // Estas funciones se usan generalmente como constructores.
    pub fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Otra función asociada,pero que toma dos argumentos:
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

pub struct Rectangle {
    pub p1: Point,
    pub p2: Point,
}

impl Rectangle {
    // Este es un método.
    // `&self` es azúcar sintáctico para `self: &Self`, donde `Self` es el tipo del
    // objeto llamador. En este caso `Self` = `Rectángulo`.
    pub fn area(&self) -> f64 {
        // `self` da acceso a los campos de estructura a través del operador punto.
        // Usamos un patrón de desestructuración:
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` es un método de `f64` que devuelve el valor absoluto del
        // llamador:
        ((x1 - x2) * (y1 - y2)).abs()
    }

    pub fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // Este método requiere que el objeto que llama sea mutable
    // `&mut self` se convierte en `self: &mut Self`.
    pub fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` posee recursos: dos enteros asignados al montón.
pub struct Pair(pub Box<i32>, pub Box<i32>);

impl Pair {
    // Este método "consume" los recursos del objeto llamador
    // `self` se convierte (se `desazucara`) en `self: Self`.
    pub fn destroy(self) {
        // Desestructura `self`:
        let Pair(first, second) = self;

        println!("Destruyendo `Pair`: Pair({}, {})", first, second);

        // `first` y `second` salen fueran del `scope` y su memoria en el montón es liberada.
    }
}

// `Closures`.
// VER: https://doc.rust-lang.org/stable/rust-by-example/fn/closures.html

// Los cierres son funciones que pueden capturar el entorno envolvente.
// Por ejemplo, un cierre que captura la `x` variable:
//  |val| val + x

// La sintaxis y las capacidades de los cierres los hacen muy convenientes para su
// uso sobre la marcha. Llamar a un cierre es exactamente como llamar a una función.
// Además tienen la capacidad de inferir los tipos de entrada y retorno; se deben especificar
// los nombres de las variables de entrada.

// Otras características de los cierres incluyen:
// • usan `||` en lugar de `()` alrededor de las variables de entrada.
// • delimitación de cuerpo opcional (`{}`) para una sola expresión (obligatoria en caso contrario).
// • la capacidad de capturar las variables del entorno externo.

// `Closures`: capturando los valores del entorno.
// VER: https://doc.rust-lang.org/stable/rust-by-example/fn/closures/capture.html

// Los cierres son intrínsecamente flexibles y harán lo que requiera la funcionalidad
// para que el cierre funcione sin anotaciones. Esto permite que la captura se adapte de manera
// flexible al caso de uso, a veces moviéndose y a veces tomando prestado. Los cierres pueden capturar variables:
// • por referencia: &T
// • por referencia mutable: &mut T
// • por valor: T

// Preferentemente capturan variables por referencia, y solo por valor cuando es requerido
// (usando el keyword `move`).

// `Closures` como parámetros de entrada.
// VER: https://doc.rust-lang.org/stable/rust-by-example/fn/closures/input_parameters.html

// Si bien Rust elige cómo capturar variables sobre la marcha en su mayoría sin anotación de
// tipo, esta ambigüedad no está permitida al escribir funciones. Al tomar un cierre como
// parámetro de entrada, el tipo completo del cierre debe anotarse utilizando uno de varios `traits`,
// y están determinados por lo que hace el cierre con el valor capturado. En orden de restricción decreciente, son:
// • `Fn`: el cierre utiliza el valor capturado por referencia (&T)
// • `FnMut`: el cierre utiliza el valor capturado por referencia mutable (&mut T)
// • `FnOnce`: el cierre utiliza el valor capturado por valor (T)

// Variable por variable, el compilador capturará las variables de la manera menos restrictiva posible.
// Por ejemplo, considere un parámetro anotado como FnOnce. Esto especifica que el cierre puede
// capturar por &T, &mut T o T, pero el compilador finalmente elegirá en función de cómo se
// usen las variables capturadas en el cierre.

// Esto se debe a que si es posible un movimiento, también debería ser posible cualquier tipo de préstamo.
// Tenga en cuenta que lo contrario no es cierto. Si el parámetro se anota como `Fn`,
// entonces no se permite capturar variables por &mut T o T.

// En el siguiente ejemplo, intente intercambiar el uso de Fn, FnMut y FnOnce para ver qué sucede.

// `Closures` y Tipos Anónimos.
// VER: https://doc.rust-lang.org/stable/rust-by-example/fn/closures/anonymity.html

// Los cierres capturan sucintamente las variables de los ámbitos adjuntos.
// ¿Tiene esto alguna consecuencia? Seguramente lo hace. Observe cómo usar un cierre
// como parámetro de función requiere genéricos , lo cual es necesario debido a cómo se definen:
/*
 * `F` debe ser genérico.
 * fn apply<F>(f: F) where
 *    F: FnOnce() {
 *    f();
 *    }
 * */

// Cuando se define un cierre, el compilador crea implícitamente una nueva estructura
// anónima para almacenar las variables capturadas en su interior, mientras implementa la
// funcionalidad a través de uno de los traits: Fn, FnMut o FnOnce para este tipo desconocido.
// Este tipo se asigna a la variable que se almacena hasta la llamada.

// Dado que este nuevo tipo es de tipo desconocido, cualquier uso en una función requerirá genéricos.
// Sin embargo, un parámetro de tipo ilimitado <T> seguiría siendo ambiguo y no estaría permitido.
// Por lo tanto, delimitar por uno de los traits: Fn, FnMut o FnOnce (que sea implementado) es suficiente para especificar su tipo.

// Funciones como `Inputs` para otras funciones.
// VER: https://doc.rust-lang.org/stable/rust-by-example/fn/closures/input_functions.html

// Dado que los cierres se pueden usar como argumentos, es posible que se pregunte si se
// puede decir lo mismo de las funciones. ¡Y de hecho pueden! Si declara una función que
// toma un cierre como parámetro, cualquier función que satisfaga el límite de rasgos de ese cierre se puede pasar como parámetro.

// `Closures` como parámetros de salida.
// VER: https://doc.rust-lang.org/stable/rust-by-example/fn/closures/output_parameters.html

// Los cierres como parámetros de entrada son posibles, por lo que también debería ser posible
// devolver cierres como parámetros de salida. Sin embargo, los tipos de cierres anónimos son,
// por definición, de tipo desconocido (`unknown`), por lo que debemos usar `impl Trait` para devolverlos.

// Los rasgos válidos para devolver un cierre son los que ya conocemos:
// • Fn
// • FnMut
// • FnOnce

// A parte esto, se debe usar la palabra clave `move`, que indica que todas las capturas ocurren por valor.
// Esto es necesario porque cualquier captura por referencia se eliminaría tan pronto como la función saliera,
// dejando referencias no válidas en el cierre.

pub fn create_fn() -> impl Fn() {
    // Podemos omitir la conversión de `&str` a `String`, que es de `propiedad` y está asignado en
    // el montón, ya que con `move` el cierre se apropia del `&str`.
    let text = "Fn".to_owned();

    // VER el error que se produce se el cierre no usa `move` apropiándose de la variable
    // capturada en:
    // file:///home/enrique/DAW/Rust/09-functions/E0373.html
    move || println!("Esto es un: {}", text)
}

pub fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();

    move || println!("Esto es un: {}", text)
}

pub fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();

    move || println!("Esto es un: {}", text)
}

// Algunos ejemplos del uso de cierres en métodos de la biblioteca standard.
// VER: https://doc.rust-lang.org/stable/rust-by-example/fn/closures/closure_examples.html

// El caso del método `any` del rasgo `Iterator` (`Trait std::iter::Iterator`).
// VER: https://doc.rust-lang.org/stable/rust-by-example/fn/closures/closure_examples/iter_any.html
// https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/std/iter/trait.Iterator.html#method.any

// `Iterator::any()` es un método del rasgo `Iterator` que, cuando se le pasa un iterador,
// regresará `true` si algún elemento satisface el predicado. `false` en caso contrario. Su firma es:

/* pub trait Iterator {
    // El tipo `asociado` sobre el que se itera.
    type Item;

    // `any` toma `&mut self`, lo que significa que el llamador puede ser prestado
    // y modificado, pero no consumido.
    fn any<F>(&mut self, f: F) -> bool
    where
        // Al ser `FnMut`, significa que cualquier variable capturada puede ser como máximo
        // modificada, no consumida. `Self::Item` indica que se necesita
        // pasar argumentos (el `Item asociado`) al cierre por valor.
        F: FnMut(Self::Item) -> bool;
} */

// Buscando a través de iteradores: El caso del método `find` del rasgo `Iterator`
// VER: https://doc.rust-lang.org/stable/rust-by-example/fn/closures/closure_examples/iter_find.html

// `Iterator::find()` es una función que itera sobre un iterador y busca
// el primer valor que satisface alguna condición. Si ninguno de los
// valores cumple la condición, devuelve `None`. Su firma es:

/* pub trait Iterator {
    // El tipo asociado sobre el que se itera.
    type Item;

    // `find` toma `&mut self`, lo que significa que el llamador puede ser prestado
    // y modificado, pero no consumido.
    fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
    where
        // Al ser `FnMut`, significa que cualquier variable capturada puede ser como máximo
        // modificada, no consumida. `&Self::Item` indica que se necesita
        // pasar argumentos al cierre por referencia, a diferencia del caso de `any()`.
        P: FnMut(&Self::Item) -> bool;
} */

// Funciones de Orden Superior (HOF).
// VER: https://doc.rust-lang.org/stable/rust-by-example/fn/hof.html

// Rust proporciona funciones de orden superior (HOF).
// Estas son funciones que toman una o más funciones y/o producen
// una función más útil. Los HOF y los iteradores perezosos le dan a
// Rust su sabor funcional.

pub fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

// Funciones Divergentes.
// VER: https://doc.rust-lang.org/stable/rust-by-example/fn/diverging.html
// VER sobre el tipo `never`:
// https://doc.rust-lang.org/std/primitive.never.html

// Las funciones divergentes nunca regresan. Se marcan con `!`, que es un tipo vacío:
/* fn foo() -> ! {
    panic!("La llamada a esta función nunca regresa.");
} */

// A diferencia de todos los demás tipos, este no se puede instanciar porque el conjunto de todos los
// valores posibles que puede tener este tipo está vacío. Tenga en cuenta que es diferente
// del `()` tipo, que tiene exactamente un valor posible.

// Por ejemplo, esta función devuelve como de costumbre, aunque no hay información en el valor devuelto.
/* fn some_fn() {
    ()
}

let _a: () = some_fn();
println!("Esta función regresa y, por lo tanto, puedes ver esta línea."); */

// A diferencia de esta función, que nunca devolverá el control al llamador.
/* let x: ! = panic!("Esta llamada nunca regresa.");
println!("Nunca podrás ver esta línea.línea"); */

// Aunque esto puede parecer un concepto abstracto, de hecho es muy útil y, a menudo, práctico. La
// principal ventaja de este tipo es que puede fundirse con cualquier otro y, por lo tanto,
// usarse en lugares donde se requiere un tipo exacto, por ejemplo, en las ramas de una
// declaración `match`.
// Esto nos permite escribir código como este:
/* fn sum_odd_numbers(up_to: u32) -> u32 {
    let mut acc = 0;
    for i in 0..up_to {
        // Ten en cuenta que el tipo de retorno de esta declaración `match` debe ser u32
        // por el tipo de la variable "addition".
        let addition: u32 = match i % 2 == 1 {
            // La variable "i" es del tipo u32, lo cual está perfectamente bien.
            true => i,
            // Por otro lado, la expresión "continue" no regresa
            // un `u32`, pero igualmente está correcta, porque nunca regresa (`!`) y por lo tanto
            // no infringe los requisitos de tipo de la expresión de coincidencia.
            false => continue,
            // `continue` salta a la siguiente iteración del `for`.
        };
        acc += addition;
    }
    acc
}

println!(
    "La suma de los impares hasta 9, excluido: {}",
    sum_odd_numbers(9)
); */
