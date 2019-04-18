package sinovatio.com

import java.sql.{Connection, PreparedStatement, ResultSet}

object impala {


  def getconnection()= {
    var conn: Connection = null
    var ps: PreparedStatement = null
    var rs: ResultSet = null

    val JDBC_DRIVER: String = "com.cloudera.impala.jdbc41.Driver"
    val CONNECTION_URL: String = "jdbc:impala://localhost:21050/safe"


    import java.sql.{DriverManager, SQLException}
    try {
      Class.forName(JDBC_DRIVER)
      conn = DriverManager.getConnection(CONNECTION_URL)
      ps = conn.prepareStatement("select network_application,count (1) from safe_dpi group by network_application;")
      rs = ps.executeQuery
      while ( {
        rs.next
      })
        System.out.println(rs.getString(1)+"\t"+rs.getString(2))
    } catch {
      case e: Exception =>
        e.printStackTrace()
    } finally try {
      if (rs != null) {

        rs.close()
      }
      if (ps != null) {

        ps.close()
      }
      if (conn != null) {

        conn.close()
      }
    } catch {
      case e: SQLException =>
        e.printStackTrace()
    }
  }
  def main (args: Array[String] ): Unit = {
   getconnection()
  }
}

