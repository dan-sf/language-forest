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


            // Do some tuple stuff
        }
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
