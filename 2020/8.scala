import scala.io.Source
import scala.collection.mutable.ArrayBuffer
import scala.collection.mutable

sealed abstract class Instruction
case class Acc(arg: Int) extends Instruction 
case class Jmp(arg: Int) extends Instruction 
case class Nop(arg: Int) extends Instruction


object Computer {
    val cmdPattern = raw"(\w{3}) ([-|+]\d+)".r

    def load(filename: String): Vector[Instruction] = {
        val lines = Source.fromFile(filename).getLines()
        val program: ArrayBuffer[Instruction] = new ArrayBuffer
        
        for (line <- lines) program += parse(line)

        program.toVector
    }

    def parse(line: String): Instruction = {

        val (cmd, arg) = cmdPattern.findFirstMatchIn(line) match {
            case Some(m) => (m.group(1), m.group(2).toInt)
            case None => throw new java.lang.IllegalArgumentException
        }
        
        cmd match {
            case "acc" => Acc(arg)
            case "jmp" => Jmp(arg)
            case "nop" => Nop(arg)
        }
    }

    def run(filename: String): Int = {
        val programOrig = load(filename)
        val replaceIndexes = findIndexes(programOrig)

        var found = false
        var result = 0
        val itIndex = replaceIndexes.iterator
        while (!found && itIndex.hasNext) {
            val index = itIndex.next()
            val program = mutateProgram(programOrig, index)
            val (f, r) = runProgram(program)
            found = f; result = r
            println(s"Index: $index. Found: $found. Result: $result")
        }

        result
    }

    def findIndexes(program: Vector[Instruction]): Vector[Int] = {
        for {
            (ins, idx) <- program.zipWithIndex 
            if ins.isInstanceOf[Jmp] || ins.isInstanceOf[Nop] 
        } yield idx
    }

    def runProgram(program: Vector[Instruction]): (Boolean, Int) = {
        var accumulator = 0
        var current = 0
        val processed = new mutable.HashSet[Int]

        while (true) {
            if (processed.contains(current)) return (false, accumulator)
            if (current == program.length) return (true, accumulator)

            processed += current
            program(current) match {
                case Acc(arg) => accumulator += arg; current += 1
                case Jmp(arg) => current += arg
                case Nop(arg) => current += 1
            }
        }

        (false, accumulator)
    }

    def mutateProgram(program: Vector[Instruction], index: Int): Vector[Instruction] = {
        val replacement = program(index) match {
            case Jmp(arg) => Nop(arg)
            case Nop(arg) => Jmp(arg)
            case Acc(arg) => Acc(arg)
        }

        program.updated(index, replacement)
    }
}

object Answer extends App {
    println("***** Start *****")

    val result = Computer.run(args(0))
    println(s"Answer is $result")

    println("***** End *****")
}