import scala.collection.mutable.ListBuffer

/**
  * Created by ivanwakeup on 1/2/18.
  */
object Day10 {

  val inputLengths = Seq(94, 84, 0, 79, 2, 27, 81, 1, 123, 93, 218, 23, 103, 255, 254, 243)

  var curPos = 0

  var skipSize = 0

  var nums = List.range(1,256)

  def main(args: Array[String]): Unit = {

    println(knotHash(inputLengths, nums))

  }

  def knotHash(lengths: Seq[Int], data: Seq[Int]): Seq[Int] = {
    var tmpSeq: Seq[Int] = data
    lengths.map(length => {

      val slices = calcSliceForData(curPos, length)
      val append = tmpSeq.slice(slices(0)._1, slices(0)._2).reverse
      val prepend = tmpSeq.slice(slices(1)._1, slices(1)._2).reverse
      val middle = tmpSeq.slice(slices(1)._2, slices(0)._1)

      tmpSeq = prepend ++ middle ++ append

      curPos = curPos + length + skipSize
      skipSize += 1

    })

    tmpSeq

  }

  def calcSliceForData(curPos: Int, length: Int): List[(Int, Int)] = {
    val tmpTotalSize = curPos + length
    val start = curPos

    val slices = new ListBuffer[(Int, Int)]()

    if(tmpTotalSize > nums.size) {
      slices.append((start, length))
      val nextEnd = tmpTotalSize - nums.size
      slices.append((0, nextEnd))
    }
    else {
      slices.append((start, start + length))
    }

    slices.toList
  }
}
