package sinovatio.com

import java.util.Properties

import org.apache.spark.sql.SQLContext
import org.apache.spark.{SparkConf, SparkContext}

object spark2 {
  def main(args: Array[String]) {

    val sparkConf = new SparkConf().setMaster("local").setAppName("spark sql test");
    val sc = new SparkContext(sparkConf);
    val sqlContext = new SQLContext(sc);
    val JDBC_DRIVER: String = "com.cloudera.impala.jdbc41.Driver"

    //1. 不指定查询条件
    //这个方式链接MySql的函数原型是：
    //我们只需要提供Driver的url，需要查询的表名，以及连接表相关属性properties。下面是具体例子：
    val url = "jdbc:impala://localhost:21050/safe";
    val prop = new Properties()
    prop.put("driver",JDBC_DRIVER)
    prop.put("user","rhino")
    prop.put("password","rhino")
    val df = sqlContext.read.jdbc(url,"tmp_addip",prop);
    df.show()
    println("第一种方法输出："+df.count());
    println("1.------------->" + df.count());
    println("1.------------->" + df.rdd.partitions.size);

    //2.指定数据库字段的范围
    //这种方式就是通过指定数据库中某个字段的范围，但是遗憾的是，这个字段必须是数字，来看看这个函数的函数原型：
    /* def jdbc(
    url: String,
    table: String,
    columnName: String,
    lowerBound: Long,
    upperBound: Long,
    numPartitions: Int,
    connectionProperties: Properties): DataFrame*/
    //前两个字段的含义和方法一类似。columnName就是需要分区的字段，这个字段在数据库中的类型必须是数字；
    //lowerBound就是分区的下界；upperBound就是分区的上界；numPartitions是分区的个数。同样，我们也来看看如何使用：
    val lowerBound = 1;
    val upperBound = 6;
    val numPartitions = 2;
    val url1 = "jdbc:mysql://192.168.0.101:3306/sas_vip?user=root&password=123456";
    val prop1 = new Properties();
    val df1 = sqlContext.read.jdbc(url1, "stock", "id", lowerBound, upperBound, numPartitions, prop1);
    println("第二种方法输出：" + df1.rdd.partitions.size);
    df1.collect().foreach(println)

    /*这个方法可以将iteblog表的数据分布到RDD的几个分区中，分区的数量由numPartitions参数决定，在理想情况下，每个分区处理相同数量的数据，我们在使用的时候不建议将这个值设置的比较大，因为这可能导致数据库挂掉！但是根据前面介绍，这个函数的缺点就是只能使用整形数据字段作为分区关键字。
这个函数在极端情况下，也就是设置将numPartitions设置为1，其含义和第一种方式一致。*/

    //3.根据任意字段进行分区
    //基于前面两种方法的限制， Spark 还提供了根据任意字段进行分区的方法，函数原型如下：
    /*def jdbc(
    url: String,
    table: String,
    predicates: Array[String],
    connectionProperties: Properties): DataFrame*/
    //这个函数相比第一种方式多了predicates参数，我们可以通过这个参数设置分区的依据，来看看例子：
    //这个函数相比第一种方式多了predicates参数，我们可以通过这个参数设置分区的依据，来看看例子：
    val predicates = Array[String]("id <= 2", "id >= 4 and id <= 5 ")
    val url2 = "jdbc:mysql://192.168.0.101:3306/sas_vip?user=root&password=123456"
    val prop2 = new Properties()
    val df2 = sqlContext.read.jdbc(url, "stock", predicates, prop2)
    println("第三种方法输出："+df2.rdd.partitions.size+","+predicates.length);
    df2.collect().foreach(println)
    //最后rdd的分区数量就等于predicates.length。


    //4.通过load获取
    //Spark还提供通过load的方式来读取数据。
    val url3 = "jdbc:mysql://192.168.0.101:3306/sas_vip?user=root&password=123456"
    val df3 = sqlContext.read.format("jdbc").option("url", url).option("dbtable", "stock").load()
    println("第四种方法输出："+df3.rdd.partitions.size);
    df.collect().foreach(println)

    sc.stop()
  }

}
