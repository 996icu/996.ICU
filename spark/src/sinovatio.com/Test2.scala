package sinovatio.com
import org.apache.spark.rdd.RDD
import org.apache.spark.{SparkConf, SparkContext}

object Test2 {
  def main(args: Array[String]): Unit = {
    val conf =new SparkConf().setMaster("local").setAppName("hahacount")
    val sc=new SparkContext(conf)
    val file=sc.textFile("src/test.txt")
    val rdd=file.flatMap(_.split(" ")).map((_,1)).reduceByKey(_+_)
    val a=rdd.collect()
    a.foreach(println)
  }
}