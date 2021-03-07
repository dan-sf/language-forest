using System;
using System.Linq;
using System.Reflection;

// Enable nullable values so we don't get warnings on our nullable examples
#nullable enable

namespace Basics {
    class Program {
        static void Main(string[] args) {
            Console.WriteLine("Lets do some stuff in C#\n");

            string message = "testing";
            Console.WriteLine($"We have some nice string interpolation: '{message}'");

            int x; x = 1234;
            var y = 456;
            int z = 789;
            Console.WriteLine($"We can declare/set variables explicitly or with type inference: {x}, {y}, {z}");

            double a = 1.1;
            int b = 2;
            double c = a + (double)b;
            int d = (int)a + b;
            Console.WriteLine("");
            Console.WriteLine($"Casting works similarly to how it is done in Java - a: {a}, b: {b}, c: {c}, d: {d}");
            Console.WriteLine("If we want to use different types together we must explicitly cast, C# won't do implicit type conversions");

            Console.WriteLine("");
            decimal dec = 0.1M;
            Console.WriteLine($"We also have decimal number type in this language: {dec}");
            Console.WriteLine("Decimal types are stored as ints with an offset for the decimal place. This can be useful for numbers that can't be stored easily in floating point or if you need to compare number with decimals in them. Which can also be error prone with floats");

            // We can define functions within functions
            static void print(string s) {
                Console.WriteLine(s);
            }
            void nl() { // Non-static functions are okay here too
                print("");
            }
            nl();

            bool tboolean = true;
            bool fboolean = false;
            print($"Booleans are lowercase in code but are printed with a leading uppercase: {tboolean} / {fboolean}");

            object integer = 10;
            object str = "testing";
            print($"Objects can be used to store any type, integer: {integer}, str: {str}");
            print($"We need to case those objects if we want to access any functions on those variables, ie str.Length will error but ((string)str).Length will work: {((string)str).Length}");
            // ^^^ Notice the cast operator has a lower precedence than the dot operator

            nl();
            dynamic dynamicInteger = 10;
            dynamic dynamicStr = "testing";
            print($"Dynamic variables are more flexible and can be used to store any type, dynamicInteger: {dynamicInteger}, dynamicStr: {dynamicStr}");
            print($"We don't need to cast dynamic variables to use their methods, they are figured out at runtime, ie dynamicStr.Length will work: {dynamicStr.Length}");

            print("Dynamic variables make the trade off of speed for type flexibility");

            nl();
            print("We can use defaults for some primitive data types");
            print($"Default int: '{default(int)}'");
            print($"Default bool: '{default(bool)}'");
            print($"Default double: '{default(double)}'");
            print($"Default string: '{default(string)}'");

            nl();

            // For nullable types we must use ? to indicate the variable is nullable
            int? nullable = null;
            print($"This value is null: {nullable}");
            nullable = 10;
            print($"This value not is null: {nullable}");

            string? nstr = null;
            int? nlen = nstr?.Length;

            print($"We can use nullable access with the ? operator. If we try to access a member of a null value it can result in null rather than an exception, nstr: '{nstr}', nlen '{nlen}'");

            var res = nstr?.Length ?? 123;
            print($"Here we use '??' (the null-coalescing operator) to assign a value if what we got is null: '{res}'");

            nl();

            var inc = 0;
            inc++;
            ++inc;
            inc += 1;
            print($"We can increment using either ++ or +=, inc: {inc}");

            nl();
            if (false) {
                print("This won't happen");
            } else if (1 == 2) {
                print("Nope");
            } else if (10 < 100) {
                print("Output from an else/if block");
            }

            {
                int scoped = 12;
                print($"We can open scopes within functions, scoped: {scoped}");
            }

            // Apparently we can't redefine this variable even though we put
            // the original definition in an enclosing scope. This is odd, I
            // would've expected this to work
            // int scoped = 21;

            nl();

            // Switch statements are pretty standard usage
            print("We can switch on values:");
            int numb = 3;
            switch (numb) {
                case 1:
                    print("We got 1");
                    break;
                case 2:
                    print("We got 2");
                    break;
                case 3:
                    print("We got 3");
                    break;
                default:
                    print("Default block");
                    break;
            }

            // Loop control flow

            nl();
            do {
                print("We can do while");
            } while (false);

            int x1 = 0;
            while (x1 < 1) {
                print("We can normal while");
                x1++;
            };

            for (int i=0; i<1; i++) {
                print("Lets for looping");
            }

            foreach (int i in new[] {1,2,3}) {
                print($"Loops yay: {i}");
            }

            nl();
            // We can use this checked block to demonstrate basic exception handling
            checked {
                int large = int.MaxValue;
                try {
                    large += 1;
                } catch (OverflowException ex) {
                    print($"We overflowed an int! We can print out the exception info - type: '{ex.GetType()}', message: '{ex.Message}'");
                }
            }

            // We can create classes an objects and use them as expected
            nl();
            var ac = new AnotherClass();
            print($"We got a value from another class defined within this namespace: {ac.x}");

            var construct = new Construct(32);
            print($"We are able to print values returned from classes: '{construct.getIt()}'");
            construct.printIt();

            nl();
            var tup = (1, 2);
            (var t0, var t1) = tup;
            print($"We can use tuples to pack vars: {tup}, and we can also unpack them: {t0}, {t1}");


            int addThem(int a = 0, int b = 0, int c = 0) {
                return a + b + c;
            }
            print($"Here we use optional named arguments: {addThem(10,10)}, {addThem(12)}, {addThem(1, 2, 7)}");

            void plusOne(ref int x) {
                x += 1;
            }

            int refAdd = 123;
            print($"We can also force functions to take variable by reference, refAdd before: {refAdd}");
            plusOne(ref refAdd);
            print($"refAdd after: {refAdd}");
            nl();

            // In C# we also have structs, the main difference between a class
            // and a struct here is that structs are allocated on the stack and
            // classes are allocated in the heap. Also, you cannot inherit from
            // a stack

            var p = new Point { x=1, y=2};
            print($"Here we have a struct point allocated on the stack: '{p}'");
            nl();

            var gs = new GettersSetters();
            print($"Getters and setters are slightly different in C#, we can define them with the get and set keywords. Lets print a default name: '{gs.Name}'");
            gs.Name = "Billy";
            print($"Lets print a name after we have set it to something: '{gs.Name}'");

            var gs2 = new GettersSetters2 { field1 = "foo", field2 = "bar" };
            var gs3 = new GettersSetters3 { field1 = "foo", field2 = "bar" };
            print($"We can use default getters and setters similarly to public attributes: gs2: '{gs2.field1} {gs2.field2}' gs3: '{gs3.field1} {gs3.field2}' ");
            gs2.field1 = "baz";
            gs2.field2 = "bop";
            gs3.field1 = "baz";
            gs3.field2 = "bop";
            print($"After setting vars: gs2: '{gs2.field1} {gs2.field2}' gs3: '{gs3.field1} {gs3.field2}' ");

            nl();
            var parent = new Parent();
            var child = new Child();
            print("We can also use inheritance if needed");
            parent.p();
            child.p();

        }
    }

    struct Point {
        public int x;
        public int y;
        public override string ToString() {
            return $"x: {x}, y: {y}";
        }
    }

    // Simple example of inheritance, we need to mark parent functions as
    // virtual if we want to override them in the child class
    class Parent {
        public virtual void p() {
            Console.WriteLine("Parent");
        }
    }

    class Child : Parent {
        public override void p() {
            Console.WriteLine("Child");
        }
    }

    class GettersSetters {
        string name = default(string);

        public string Name {
            get { return name; }
            set { name = value; }
        }
    }

    // The two classes are basically the same, here we show how the default
    // get/set commands work
    class GettersSetters2 {
        public string field1 { get; set; }
        public string field2 { get; set; }
    }
    class GettersSetters3 {
        public string field1;
        public string field2;
    }


    class AnotherClass {
        public int x = 123;
    }

    class Construct {
        int val;

        public Construct(int val) {
            this.val = val;
        }

        public void printIt() {
            Console.WriteLine($"This is the val: '{val}'");
        }

        public int getIt() {
            return this.val;
        }
    }
}
