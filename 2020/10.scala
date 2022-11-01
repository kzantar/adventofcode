import scala.io.Source

object Answer extends App {
    val adapters = Source.fromFile(args(0)).getLines().map(_.toInt).toArray
    val builtin = adapters.max + 3
    val outlet = 0

    val jolts = outlet +: adapters :+ builtin
}