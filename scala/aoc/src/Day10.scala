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

    calcPosition()

  }


  def calcPosition(): List[Int] = {

    inputLengths.map(length => {

      println(s"starting loop, nums is now: $nums \n")
      val reversed = nums.slice(curPos, curPos + length + 1)
        .reverse
      println(s"reversed is: $reversed \n")
      val append = nums.slice(curPos + length + 1, nums.size + 1)
      println(s"append is: $append \n")
      nums = List.concat(reversed, append)

      if (nums.size - (curPos + length + skipSize) <= 0) {

        curPos = (curPos + length + skipSize) - nums.size
        println(s"curPos size reached, now is: $curPos \n")
      }
      else {
        curPos = curPos + length + skipSize
        println(s"incrementing curPos normally and is now: $curPos \n")
      }

      skipSize += 1

      println(s"nums is now: $nums")
    }
    )
    return nums
  }

  def calcSlice(curPos: Int, skipSize: Int, length: Int): List[(Int, Int)] = {
    val tmpTotalSize = curPos + length
    val start = curPos

    val slices = new ListBuffer[(Int, Int)]()

    slices.append((start, length))

    if(tmpTotalSize > nums.size) {
      val nextEnd = tmpTotalSize - nums.size
      slices.append((0, nextEnd))
    }

    slices.toList
  }
}
