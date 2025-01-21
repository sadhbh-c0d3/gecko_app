import scala.collection.mutable.HashMap
import scala.concurrent.ExecutionContext.Implicits.global
import scala.concurrent.{ExecutionContext, Future}
import scala.util.{Failure, Success}
import cats.data.Either
import cats.implicits._

case class GeckoMarket(value: String) {
  override def toString: String = s"GeckoMarket($value)"
}

case class GeckoPrice(value: Double) {
  override def toString: String = s"GeckoPrice($value)"
}

case class GeckoQuantity(value: Double) {
  override def toString: String = s"GeckoQuantity($value)"
}

sealed trait GekoSide
case object Ask extends GekoSide
case object Bid extends GekoSide

object GekoSide {
  def fromString(str: String): Either[String, GekoSide] = str match {
    case "ask" => Right(Ask)
    case "bid" => Right(Bid)
    case _ => Left(s"Invalid GekoSide: $str")
  }
}

case class GekoData(market: GeckoMarket, price: GeckoPrice, quantity: GeckoQuantity, side: GekoSide)

// Simulate a database connection (replace with actual DB connection logic)
case class DbConnection()

// Adapter struct to hold configuration and database connection
case class GeckoAdapter(dbConnection: DbConnection, marketMappings: MarketMappings)

// Introduce GekoApplication struct
case class GekoApplication(adapter: GeckoAdapter)

object GekoApplication {
  def apply(config: GekoConfig): GekoApplication = {
    val dbConnection = DbConnection()
    GekoApplication(GeckoAdapter(dbConnection, loadMarketMappings(config, dbConnection)))
  }
}

case class MarketMappings(forward: Map[String, String], reverse: Map[String, String])

case class GekoConfig(dbHost: String, dbPort: Int, markets: List[String])

object Main extends App {
  println("Hello!")

  implicit val executionContext: ExecutionContext = global

  lazy val application: GekoApplication = {
    val config = GekoConfig("localhost", 5432, List("ARBTC", "ETHBTC"))
    GekoApplication(config)
  }

  def loadMarketMappings(config: GekoConfig, dbConnection: DbConnection): MarketMappings = {
    // This function would typically interact with a database 
    // to fetch the market mappings using db_connection and config.
    // Here, we simulate this with hardcoded values based on config.markets

    val forward = config.markets.map(market => market -> s"$market/BTC").toMap
    val reverse = forward.map(_.swap)

    MarketMappings(forward, reverse)
  }

  def parseGekoData(jsonStr: String): Either[String, GekoData] = {
    import io.circe._, io.circe.generic.auto._, io.circe.parser._

    decode[GekoData](jsonStr)
  }

  def testMarketdata(): Unit = {
    val jsonStr =
      """
        |{
        |  "market": "ARBTC",
        |  "price": "0.0012",
        |  "quantity": "10.5",
        |  "size": "bid"
        |}
        |""".stripMargin

    parseGekoData(jsonStr) match {
      Right(data) => {
        assert(data.market.value == "ARBTC/BTC")
        assert(math.abs(data.price.value - 0.0012) < 1E-6) // Safer comparison for floating-point numbers
        assert(math.abs(data.quantity.value - 10.5) < 1E-6) // Safer comparison for floating-point numbers
        assert(data.side == Bid)
      }
      Left(error) => println(s"Error parsing JSON: $error")
    }
  }

  // Uncomment to run the test
  // testMarketdata()
}