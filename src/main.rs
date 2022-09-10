// #![feature(never_type)]

use functions::{
    create_fn, create_fnmut, create_fnonce, fizzbuzz_to, is_odd, Pair, Point, Rectangle,
};
use std::mem;

fn main() {
    // 09-Funciones.
    // =============================================================
    // VER: https://doc.rust-lang.org/stable/rust-by-example/fn.html

    // A diferencia de C/C++, no hay restricciones en el orden de
    // las definiciones de funciones.
    // Podemos usar esta función aquí y definirla en algún lugar
    // más adelante.
    fizzbuzz_to(100);

    // Funciones y Métodos asociados
    // VER: https://doc.rust-lang.org/stable/rust-by-example/fn/methods.html

    // Algunas funciones están conectadas a un tipo particular. Estos vienen en
    // dos formas: funciones asociadas y métodos. Las funciones asociadas son funciones
    // que generalmente se definen en un tipo, mientras que los métodos son funciones
    // asociadas que se llaman en una instancia particular de un tipo.
    // (NOTA: las funciones asociadas equivaldrían en OOP a los métodos estáticos.)

    let rectangle = Rectangle {
        // Las funciones asociadas son llamadas usando dos puntos dobles (`::`).
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Los métodos se llaman usando el operador punto
    // Tenga en cuenta que el primer argumento `self` se pasa implícitamente,
    // es decir, `rectangle.perimeter()` === `Rectangle::perimeter(rectangle)`
    println!("Perímetro del rectángulo: {}", rectangle.perimeter());
    println!("Área del rectángulo: {}", rectangle.area());

    // Si tratamos de usar la varible `rectangle`, que no ha sido declarada como mutable,
    // se producirá un error: `¡no se puede prestar «rectangle» como mutable!`
    // rectangle.translate(1.0, 0.0);

    // Creamos una instancia mutable de `Rectangle`:
    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Los objetos mutables pueden llamar a métodos mutables:
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // ¡ERROR! Después de llamar a `destroy()` se ha "consumido" `pair`,
    // por lo que no se puede hacer nada con él, p.ej., llamar otra vez
    // a `destroy()` sobre dicha instancia (`uso de un valor movido`):
    // pair.destroy();

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

    // Incremento vía cierres y funciones.
    fn function(i: i32) -> i32 {
        i + 1
    }

    // Los cierres son `funciones` anónimas.
    // Aquí los `bindeamos` o vinculamos `por referencia` (como se suele hacer en JS).
    // La anotación es idéntica a la de las funciones, pero es opcional
    // el uso de `{}` para envolver el cuerpo.
    // En este caso, estas `funciones sin nombre` se las asignamos a variables
    // con nombres apropiados:
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    // Veremos que el error que vemos por la falta de anotación de tipo,
    // desaparece cuando usamos el `closure`, lo que demuestra su capacidad de inferencia de tipos:
    let closure_inferred = |i| i + 1;

    let i = 1;

    // Llamando las funciones y a los `closures`:
    println!("`function`: {}", function(i));
    println!("`closure_annotated`: {}", closure_annotated(i));
    println!("`closure_inferred`: {}", closure_inferred(i));

    // Un cierre sin argumentos que devuelve un `i32`.
    // Se infiere automáticamente el tipo de retorno.
    let one = || 1;
    println!("`Closure` que devuelve `1`: {}", one());

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

    let color = String::from("verde");

    // Un cierre para imprimir `color` que inmediatamente toma prestado (`&`) `color` y
    // almacena el préstamo y el cierre en la variable `print`. Permanecerá
    // prestado hasta que `print` se use por última vez.
    //
    // `println!` solo requiere argumentos por referencia inmutable, por lo que no
    // se imponen una fuerte restricción:
    let print = || println!("`color` es: {}", color);

    // Llamando al `closure` usando el préstamo:
    print();

    // `color` se puede tomar prestado inmutablemente de nuevo, porque el cierre solo mantiene
    // una referencia inmutable a `color`.
    let _reborrow = &color;
    print();

    // Se puede mover o volver a tomar prestado después
    // de un uso final de `print()`:
    let _color_moved = color;

    let mut count = 0;

    // Un cierre para incrementar `count` podría tomar `&mut count` o `count`
    // pero `&mut count` es menos restrictivo, así que toma eso.
    //
    // Se requiere un `mut` en `inc` porque un `&mut` está almacenado dentro. De este modo,
    // llamar al cierre muta el cierre que requiere un `mut`.
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // Llamando a `closure` usando el préstamo mutable:
    inc();

    // El cierre aún tiene prestado `count` porque lo vamos
    // a volver a llamar más tarde.
    // Un intento de volver a pedir prestado conduciría a un error,
    // porque las reglas de préstamo impiden tener en el mismo alcance
    // un préstamo inmutable y otro mutable a la vez:
    // let _reborrow = &count;
    inc();

    // El cierre ya no necesita tomar prestado `&mut count`. Por tanto, es
    // posible volver a pedir prestado sin un error (independientemente de que sea mutable o
    // inmutable):
    let _count_reborrowed = &mut count;

    // Un tipo que no es `copiable`
    let movable = Box::new(3);

    // `mem::drop` requiere `T` por lo que debe tomar por valor.
    // Un tipo que fuera `copiable`, es decir, que implementase el rasgo `Copy`,
    // copiaría en el cierre dejando intacto el original.
    // Un tipo no copiable debe moverse, por lo que `movable` se mueve inmediatamente al cierre.
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` consume la variable por lo que solo se puede llamar una vez.
    // Es decir, el cierre implementa automáticamente el rasgo `FnOnce`:
    consume();
    // Intentar volver a llamar al cierre `consume()`, provacaría un error:
    // `Uso de un valor después de ser movido`.
    // consume();
    // `consume` imprime el valor que está dentro del `Box` gracias a la coerción de `deref`
    // que efectúa `println!`.

    // El uso del keyword `move` antes de las tuberías del cierre (`pipes`).
    // fuerza siempre a tomar propiedad de las variables capturadas.

    // `Vec` tiene semánticamente la no `copiabilidad`.
    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // Si ahora tratamos de usar `haystack`, p.ej. intentando conocer su `lenght`
    // resultaría un error, pues el comprobador de préstamos nos impide reusar
    // una variable después de haber sido movida:
    // println!("Hay {} elementos en `haystack`", haystack.len());
    // Pero si eliminamos `move` de la firma del cierre, este tomará prestada a la variable
    // `haystack` inmutablemente, por lo que esta siguirá estando disponible y descomentar
    // la línea anterior no dará error.

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

    // Una función que toma un cierre como argumento y lo llama.
    // <F> indica que F es un "parámetro de tipo genérico"
    fn apply<F>(f: F)
    where
        // El cierre no recibe entrada y no devuelve nada.
        F: FnOnce(),
        // ^ TODO: Intenta cambiar esto a `Fn` o `FnMut`.
    {
        f();
    }

    // Una función que toma un cierre y devuelve un `i32`.
    fn apply_to_3<F>(f: F) -> i32
    where
        // El cierre toma un `i32` y devuelve un `i32`.
        F: Fn(i32) -> i32,
    {
        f(3)
    }

    let greeting = "hello";
    // Un tipo no copiable.
    // `to_owned()` crea datos propios (String) a partir de uno prestado:
    let mut farewell = "goodbye".to_owned();

    // Captura 2 variables: `greeting` por referencia y
    // `farewell` por valor.
    let diary = || {
        // `greeting` es tomado por referencia: requiere `Fn`:
        println!("Dije {}.", greeting);
        // La mutación obliga a `farewell` a ser capturado por
        // referencia mutable. Ahora requiere `FnMut`.
        farewell.push_str("!!!");
        println!("Luego grité {}.", farewell);
        println!("Ahora puedo dormir. zzzzz");

        // Llamamos manualmente a `drop` por lo que forzamos a que `farewell`
        // sea capturado por valor. Ahora requiere `FnOnce`.
        mem::drop(farewell);
    };
    // En resumen, el cierre `diary` podría ser `Fn`, `FnMut` y, finalmente, `FnOnce`.
    // Esto último, obliga a que el cierre, al tomar posesión de la varible capturada,
    // solo pueda ser llamado una vez (como indica su nombre).

    // Llamamos a la función que aplica el cierre antes definido:
    apply(diary);
    // Vemos que `diary` cumple con la restricción de ser `FnOnce`,
    // por lo que no lo podemos llamar otra vez: ¡Error!: `diary` hace uso de un valor movido.
    // apply(diary);
    // Si cambiamos el requerimiento de `apply` de un cierre `FnOnce` uno `FnMut`,
    // el compilador nos dirá que `diary` implementa `FnOnce` (por que se apropia de `farewell`),
    // y no `FnMut`, como requiere `apply`.
    // Algo parecido ocurrirá si hacemos que `apply` requiera `Fn`: pero ahora tampoco
    // podemos mutar `farewell`, porque el rasgo `Fn` solo permite referencias inmutables.

    // `double` satisface el límite de rasgo que requiere `apply_to_3`:
    let double = |x| 2 * x;

    println!("El doble de 3 es: {}", apply_to_3(double));
    // `apply_to_3` toma un cierre que implementa `Fn`, por lo que solo requiere referencias
    // inmutables. Graciias a la coerción de `deref`, si le pasamos un `i32`, fuerza un `&i32`,
    // por lo que podemos llamar a `apply_to_3` otra vez:
    println!("El doble de 3 es: {}", apply_to_3(double));

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

    // `F` debe implementar `Fn` para un cierre que no requiere
    // inputs y no devuelve nada, exactamente lo que se requiere
    // para `print`.
    fn apply1<F>(f: F)
    where
        F: Fn(),
    {
        f();
    }

    let x = 7;

    // Captura `x` en un tipo anónimo e implementa
    // `Fn` para ello. Lo almacena en `print`.
    let print = || println!("{}", x);

    apply1(print);

    // Funciones como `Inputs` para otras funciones.
    // VER: https://doc.rust-lang.org/stable/rust-by-example/fn/closures/input_functions.html

    // Dado que los cierres se pueden usar como argumentos, es posible que se pregunte si se
    // puede decir lo mismo de las funciones. ¡Y de hecho pueden! Si declara una función que
    // toma un cierre como parámetro, cualquier función que satisfaga el límite de rasgos de ese cierre se puede pasar como parámetro.

    // Definir una función que toma un argumento `F` genérico
    // delimitado por `Fn`, y lo llama:
    fn call_me<F: Fn()>(f: F) {
        f();
    }

    // Definir una función contenedora que satisfaga el límite `Fn`:
    fn function1() {
        println!("¡Soy una función!");
    }

    // Definir un cierre que también satisfaga el límite `Fn`:
    let closure = || println!("¡Soy un cierre!");

    call_me(closure);
    call_me(function1);

    // `Closures` como parámetros de salida.
    // VER: https://doc.rust-lang.org/stable/rust-by-example/fn/closures/output_parameters.html

    // Los cierres como parámetros de entrada son posibles, por lo que también debería ser posible
    // devolver cierres como parámetros de salida. Sin embargo, los tipos de cierre anónimos son,
    // por definición, desconocidos, por lo que debemos usar `impl Trait` para devolverlos.

    // Los rasgos válidos para devolver un cierre son los que ya conocemos:
    // • Fn
    // • FnMut
    // • FnOnce

    // A parte esto, se debe usar la palabra clave `move`, que indica que todas las capturas ocurren por valor.
    // Esto es necesario porque cualquier captura por referencia se eliminaría tan pronto como la función saliera,
    // dejando referencias no válidas en el cierre.

    // `Closures` como parámetros de salida de funciones.
    // VER: https://doc.rust-lang.org/stable/rust-by-example/fn/closures/output_parameters.html

    let fn_plain = create_fn();
    // `fn_mut` tiene que ser declarada mutable, porque `create_fnmut`
    // devuelve algo que implementa `FnMut`. Al llamar a `fn_mut()` es
    // como si se pidiera prestada mutablemente, pero `fn_mut` ha sido
    // declarada inmutable:
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();

    // Si ahora intentamos llamar a `fn_plain` y `fn_mut` nuevamente,
    // no habrá problemas, pero si intentamos llamar otra vez a
    // `fn_once` compilará con error, porque el valor fue movido:
    // el movimiento ocurre porque `fn_once` tiene el tipo `impl FnOnce()`, que no implementa el rasgo `Copiar`.
    // `fn_once` movido debido a esta llamada; valor utilizado aquí después del movimiento
    // nota: este valor implementa `FnOnce`, lo que hace que se mueva cuando se le llama.
    fn_plain();
    fn_mut();
    // fn_once();

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

    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` para vecs produce `&i32`, pero lo desestructuramos a `i32`, que es lo que necesita
    // el cierre. Sería equivalente a pasarle este cierre: `|x| *x == 2)`. Esto es debido a que no
    // se puede comparar una referencia con un primitivo, así que hacemos una desestructuración con
    // `|&x| x ==2` o hacemos una desreferenciación con `|x| == *x == 2`
    // VER: https://stackoverflow.com/questions/34733811/what-is-the-difference-between-iter-and-into-iter
    println!("¿Está `2` en `vec1`: {}", vec1.iter().any(|&x| x == 2));
    // `into_iter()` para vecs produce `i32`. No requiere desestructuración.
    // Hay que tener en cuenta que `into_iter()` no entrega referencias sino los valores,
    // lo que significa que mueve el vector (VER: https://doc.rust-lang.org/std/vec/struct.IntoIter.html)
    // al `struct IntoIter`, a diferencia de `iter()` que si entraga un `struct` de referencias
    // (VER: https://doc.rust-lang.org/std/slice/struct.Iter.html);
    // esto quiere decir que en el primer caso no podemos volver a usar el vector.
    println!("¿Está `2` en `vec2`: {}", vec2.into_iter().any(|x| x == 2));
    // ¡Error! Prestando un valor que ha sido movido por el método `into_iter()`
    // println!("Usando `vec2` de nuevo: {:?}", vec2);

    // Lo mismo es aplicable para arrays:
    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];
    println!("¿Está `2` en `array1`: {}", array1.iter().any(|&x| x == 2));
    println!(
        "¿Está `2` en `array2`: {}",
        array2.into_iter().any(|x| x == 2)
    );

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

    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` para vecs produce `&i32`.
    let mut iter = vec1.iter();
    // `into_iter()` para vecs produce `i32`.
    let mut into_iter = vec2.into_iter();

    // `iter()` para vecs produce `&i32`, y queremos hacer referencia a uno de sus
    // elementos, por lo que tenemos que desestructurar `&&i32` a `i32`.
    // Entonces como `find()` toma una referencia a cada uno de los items (ver la firma del método)
    // pero lo que nos ha devuelto `iter()` también es una referencia a los items, lo que tenemos
    // que manejar en el cierre es una *UNA REFERENCIA A UNA REFERENCIA*, que obviamente, no
    // podemos comparar con un primitivo `i32`. Por ello nos vemos obligados a desectructurar
    // un doble ampersand (`&&`).
    // Se podría hacer lo mismo con una doble desreferenciación: `|x| **x == 2`
    println!("Encontrado `2` en `vec1`: {:?}", iter.find(|&&x| x == 2));
    // `into_iter()` para vecs produce `i32`, y queremos hacer referencia a uno de
    // sus elementos, por lo que tenemos que desestructurar `&i32` a `i32`.
    // En el caso de `into_iter()`, basta un solo ampersand para hacer la desectructuración,
    // o una simple desreferenciación: `|x| *x ==2`
    println!(
        "Encontrado `2` en `vec2`: {:?}",
        into_iter.find(|&x| x == 2)
    );

    // Lo mismo es aplicable para arrays:
    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];
    println!(
        "¿Encontrado `2` en `array1`: {:?}",
        array1.iter().find(|&&x| x == 2)
    );
    println!(
        "¿Está `2` en `array2`: {:?}",
        array2.into_iter().find(|&x| x == 2)
    );

    // Iterator::find te da una referencia al item. Pero si deseas el índice del item, usa Iterator::position.
    // VER: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.position

    let vec = vec![1, 9, 3, 3, 13, 2];

    // `iter()` para vecs produce `&i32`, pero el cierre que le tenemos que pasar a `position()` no toma referencias,
    // por lo que tenemos que desestructurar `&i32` a `i32`.
    let index_of_first_even_number = vec.iter().position(|&x| x % 2 == 0);
    println!(
        "Índice del primer número par: {}",
        index_of_first_even_number.unwrap()
    );

    // `into_iter()` para vecs produce `i32` y `position()` no toma una referencia, entonces
    // no tenemos que desestructurar.
    let index_of_first_negative_number = vec.into_iter().position(|x| x < 0);
    println!(
        "Índice de primer número negativo: {:?}",
        index_of_first_negative_number
    );

    // Rust proporciona funciones de orden superior (HOF).
    // Estas son funciones que toman una o más funciones y/o producen
    // una función más útil. Los HOF y los iteradores perezosos le dan a
    // Rust su sabor funcional.

    println!("Encuentra la suma de todos los números impares al cuadrado menores de 1000.");

    let upper = 1000;

    // *** Enfoque imperativo ***
    // Declarar variable acumuladora:
    let mut acc = 0;
    // Iterar: 0, 1, 2, ... hasta el infinito
    for n in 0.. {
        // Cuadrado de `n`
        let n_squared = n * n;
        if n_squared >= upper {
            // Rompe el ciclo si se excede el límite superior.
            break;
        } else if is_odd(n_squared) {
            // Acumular valor, si es impar:
            acc += n_squared;
        }
    }

    println!("Resultado obtenido en estilo imperativo: {}", acc);

    // *** Aproximación funcional ***
    // VER las definiciones de los métodos que se usan seguidamente:
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.take_while
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold
    let sum_of_squared_odd_numbers: u32 = (0..)
        .map(|n| n * n) // Todos los números naturales al cuadrado
        .take_while(|&n_squared| n_squared < upper) // Por debajo del límite superior
        .filter(|&n_squared| is_odd(n_squared)) // Que sean impares
        .fold(0, |acc, n_squared| acc + n_squared); // Acumúlalos mediante un cierre acumulador

    println!(
        "Resultado obtenido en estilo funcional: {}",
        sum_of_squared_odd_numbers
    );

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
    fn sum_odd_numbers(up_to: u32) -> u32 {
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
    );
}
