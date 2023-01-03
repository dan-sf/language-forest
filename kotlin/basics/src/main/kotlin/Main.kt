
// A small program to highlight the basics of the kotlin programming language

// This does not cover advanced OOP/functional/async features, or things like
// scope functions (see: https://kotlinlang.org/docs/scope-functions.html).
// This is only meant as a quick intro to the basics of the language

fun main(args: Array<String>) {

    // Output to console
    print("We can use both print, ")
    println("and println functions to output to the console either exluding/including an ending newline")

    val x = 10
    println("This is a constant val: $x") // We can string interp with $ or ${}

    var y = 13
    println("This is a variable var: $y")
    y = 8
    println("Now the var is: $y")

    val z: String = y.toString()
    println("We can cast to a string using toString function similarly to how it works in java, this is a string: $z")


    println()
    println("We can create new functions within the main method and run them")
    fun runIt() { // The default return type for functions is Unit. This function is equivalent: 'fun runIt(): Unit'
        println("We are running it")
    }
    runIt()

    // Functions can be used as args/returned from other functions
    fun runItAgain(f: () -> Unit) {
        println("We are going to run it again")
        f()
    }
    runItAgain(::runIt) // We need to use '::' to let kotlin know that we are passing the function as a reference
    runItAgain({ runIt() }) // We can also pass the function like this, basically passing just the function body

    println()

    // Kotlin deals with nullable types using the ? operator

    var nullString: String? = null
    if (nullString == null) {
        nullString = "testing"
    }
    println("Our string is no longer null: $nullString")
    val notNullString = nullString!!
    println("We can use the '!!' operator to assert a type is not null and convert it to a non-null type: $notNullString")

    println()
    println("Kotlin supports vararg functions as well")
    fun varargFunction(vararg args: Int) {
        println("Argument has ${args.size} elements")
    }
    varargFunction()
    varargFunction(1)
    varargFunction(1, 2, 3)

    println()

    // For loops

    println("We can make use of a for loop, with the 0..5 range syntax. The range is inclusive")
    for (i in 0..5) {
        print("$i, ")
    }
    println()
    println("We can use until to make the range non-inclusive")
    for (i in 0 until 5) {
        print("$i, ")
    }
    println()
    println("Lets go backwards")
    for (i in 5 downTo 0) {
        print("$i, ")
    }
    println()
    println("Lets go in steps of 2")
    for (i in 0..5 step 2) {
        print("$i, ")
    }
    println()

    println("While loops basically work the same as in most other languages, kotlin also supports the do while syntax as well")
    var i = 0
    while (i < 3) {
        print("$i, ")
        i += 1
    }
    do {
        print("do $i, ")
        i += 1
    } while (i < 3)

    println()

    // When syntax
    println("The when function is similar to a match/case statement")
    fun whenIt(x: Any) {
        when (x) {
            1 -> println("x == 1")
            2 -> println("x == 2")
            "foo" -> println("x == foo")
            "bar" -> println("x == bar")
            else -> {
                println("We don't know what x is...")
            }
        }
    }
    whenIt(1)
    whenIt(2)
    whenIt("foo")
    whenIt("bar")

    println()

    println("Here we create a new class, notice that the constructor args are accessible members automatically")
    class MyClass(val x: Int) {
        fun addFunction(y: Int): Int {
            return x + y
        }
    }

    val myClass = MyClass(10)
    println("This is myClass.x: ${myClass.x}")
    println("This is myClass.addFunction(12): ${myClass.addFunction(12)}")

    println()

    println("For classes that only hold data, the data class construct can be used")
    data class PointData(val x: Int, val y: Int, val z: Int)

    val point = PointData(1, 2, 3)
    println("Here is my point data class: $point") // Notice printing data classes is already formatted well

    var pointB = point.copy()
    println("Data classes can be copied: $pointB")

    pointB = point.copy(z = 18)
    println("And can be copied with updating only some values: $pointB")

    println()

    // Some basic types

    var myMap = mapOf("key1" to 10, "key2" to 20)
    println("We also get a hash map data type: $myMap. We can iterate on it as well")
    for ((k, v) in myMap) {
        println("K: $k, V: $v")
    }

    var myList = listOf(1, 2, 3)
    println("We also get a list data type: $myList. We can iterate on it as well")
    for (it in myList) {
        print("it: $it, ")
    }
    println()

    var mySet = setOf(1, 2, 3, 3, 3)
    println("We also get a set data type: $mySet. We can iterate on it as well")
    for (it in mySet) { // Notice we only get one 3 here
        print("it: $it, ")
    }
    println()

    val seq = generateSequence(1, {it + 1})
    println("Kotlin also has sequences which are lazy: ${seq.take(4).toList()}")

    println()

    // Functional stuff
    var functional = (0..10)
        .map({it + 2}) // Note, you can omit the parens here: '.map {it + 2}'
        .filter({it > 6})
        .toList()
    println("Lets get functional: $functional")

}

