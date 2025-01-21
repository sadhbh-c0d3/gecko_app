## Example trading app - Market data

Let's write market data adapter in Rust!

### Disclaimer

_**Note**
Gecko is an imaginary crypto exchange, and not real, and any 
similarity to any existing exchange or data is not intentional._

### AI Coding Assistance

We have asked Gemini to define data types, implement serde traits,
and then we had conversation with Gemini to improve the code.

_**Note**
You can watch the whole experience where I ask Gemini to
perform coding tasks on my YouTube channel_

We used our long time experience as software engineer to form
tasks for Gemini to perform. 

### Does it run?

Yes, you can use `cargo test` to build and run test.

### Goal of this exercise

See how we would implement basic market data structure in Rust
for an imaginary crypto exchange.

### Let's do it!

First thing first, let's define some types that correspond to what
exchange is sending us.

We'll define `GeckoData`, which has `market`, `price`, `quantity`, 
and `side`.

We'll define types for each of those fields as well: `GeckoMarket`,
`GeckoPrice`, `GeckoQuantity`, and `GeckoSide`. This way we will be
able to (de)serialize those fields in an easy way using serde, and
as well we'll have clear understanding of what data format does the
Gecko exchange require, and what data format we work with internally.

For instance Gecko echxchange will send us price as decimal string,
and we prefer to use `f64` internally. We understand that `f64` will
introduce floating point imprecisions, but we are happy to accept that.
The `GeckoPrice` will define `Serialize` and `Deserialize` traits so
that decimal string is converted to(from) our intenral `f64` data type.

The `GeckoQuantity` is similar to `GeckoPrice`, and it corresponds to 
the quantity of the traded crypto. Now, it is good observation that
both quantity and price are similar in their meaning.
For example if I buy 2 BTC for price of 12 ETH each, then to receive
2 BTC, I need to deliver 24 ETH, because 2 * 12 = 24, and 24 ETH is the 
amount deliverable, while 2 BTC is the amount I will receive. Note that
number 2, 12, and 24 in this equation are the quantities. If we add
asset symbol after the number e.g. 2 BTC, 12 ETH or 24 ETH, then this
is the amount, and then the amount of the one asset we need to pay to
obtain one unit of other asset is called price. Sometimes amount and
quantity are used interchangeably, as the symbol of the asset can be
deducted from the context. Since price is the amount to obtain one unit
of an asset, then type of the price is the same as type of an amount,
which can be same as the type of the quantity. These are some design
decisions we need to make when designing trading application.

We also define `GeckoMarket` type even though it is a `String` and 
internally we also use `String`. However there is a difference in
how this string is formatted internally and how it is formatted by
Gecko exchange. For example for traded pair (AR, BTC) the Gecko
symbol is `ARBTC` while our internal symbol is `AR/BTC`. We define
`GeckoMarket` as a type for which we implement `Serialize` and 
`Deserialize` traits, so that a conversion of the value happens. Note
that it is not trivial conversion, and we use global `HashMap` with
pre-loaded mappings between symbols Gecko vs internal.

There is markets and assets traded on the exchange, and each of them
has some symbol assigned. An asset is a crypto-currency, and would
have a symbol like BTC, ETH, or AR, etc. The market on the other hand
involves traded pair, and would have symbol like ARBTC, ETHBTC, etc.
We can see that symbol is just few letter label that is given to
either traded pair or asset. The market gets its symbol from traded
pair, as we trade that pair on that market. Sometimes market has
different meaning, e.g spot, margin, futures, etc.
Here in our example code `GeckoMarket` is a market of single traded
pair, e.g. ARBTC.

We also define `GeckoSide` as an enum type of `Bid` and `Ask`. Some 
exchanges might use Buy and Sell terms instead, or use Bid and Ask
for sided of the order book, and Buy and Sell for orders. By having
`GeckoSide` type we isolate ourselves from the exchange specific
terms and we can convert that into our internal Bid and Ask. Note 
that here in our example code we don't have internal enum type 
defined, and we just define Bid and Ask directly in `GeckoSide`.

Now we can easily convert data between Gecko data format and our
internal data format by using `GeckoData` and related `GeckoMarket`,
`GeckoPrice`, `GeckoQuantity`, and `GeckoSide`.