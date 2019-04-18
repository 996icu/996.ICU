package sinovatio.com

import java.util.Properties

import org.apache.spark.{SparkConf, SparkContext}
import org.apache.spark.sql.SQLContext

//读取impala数据库，用spark的类
object spark1 {
  def readimpala(SQLContext: SQLContext)={
    val JDBC_DRIVER: String = "com.cloudera.impala.jdbc41.Driver"
    val CONNECTION_URL: String = "jdbc:impala://localhost:21050/safe"
    val table="tmp_addip"
    val properties = new Properties()
//    添加驱动
    properties.put("user","cloudera")
    properties.put("password","cloudera")
    properties.put("driver",JDBC_DRIVER)

//    读取impala的数据
    println("读取数据：")

//    使用dataframe
    val dataFrame = SQLContext.read.jdbc(CONNECTION_URL,table,properties).select("time","ip")
    println(dataFrame.count())

  }

  def main(args: Array[String]): Unit = {
    val sc = new SparkContext(new SparkConf().setMaster("local").setAppName("con"))
    val sQLContext = new SQLContext(sc)
//    调用方法
    readimpala(sQLContext)
    sc.stop()

  }
}
