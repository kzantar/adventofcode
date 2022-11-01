import scala.io.Source

object Rules {
    def byr(value: String) = {
        raw"\d{4}".r.matches(value) &&
        value.toInt >= 1920 &&
        value.toInt <= 2002
    }

    def iyr(value: String) = {
        raw"\d{4}".r.matches(value) &&
        value.toInt >= 2010 &&
        value.toInt <= 2020
    }

    def eyr(value: String) = {
        raw"\d{4}".r.matches(value) &&
        value.toInt >= 2020 &&
        value.toInt <= 2030
    }

    def hgt(value: String) = {
        val in = raw"(\d+)in".r
        val cm = raw"(\d+)cm".r

        val result = value match {
            case in(v) => v.toInt >= 59 && v.toInt <= 76
            case cm(v) => v.toInt >= 150 && v.toInt <= 193
            case _ => false
        }

        result
    }

    def hcl(value: String) = {
        raw"#[0-9a-f]{6}".r.matches(value)
    }

    def ecl(value: String) = {
        Set("amb", "blu", "brn", "gry", "grn", "hzl", "oth").contains(value)
    }

    def pid(value: String) = {
        raw"\d{9}".r.matches(value)
    }
}


object Answer extends App {
    def batch2map(batch: String): Map[String, String] = {
        val pairs = for (
            Array(k, v) <- batch.split(Array(' ', '\n')).map(_.split(':'))
        ) yield (k, v)
        
        pairs.toMap
    }

    def isValid(m: Map[String, String]): Boolean = {
        val diff = required.keySet.diff(m.keySet) - "cid"
        if (!diff.isEmpty) return false

        val result = for ((key, rule) <- required)
            yield rule(m(key))
        
        result.forall(_ == true)
    }

    val required = Map(
        "byr" -> Rules.byr _, 
        "iyr" -> Rules.iyr _, 
        "eyr" -> Rules.eyr _, 
        "hgt" -> Rules.hgt _, 
        "hcl" -> Rules.hcl _, 
        "ecl" -> Rules.ecl _, 
        "pid" -> Rules.pid _,
    )
    val input = Source.fromFile("4_input.txt").mkString
    val batches = input.split("\n\n")

    val result = batches.map(batch2map).map(isValid)

    println(result.count(_ == true))
}