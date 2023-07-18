# stock fund dapp

## Stock股票

您好，一个完整定义的表结构如下：

| Field       | Type    | Description                      |
|-------------|---------|----------------------------------|
| id          | int     | Record identifier, auto-increment|
| stock_code  | varchar | Stock code, alphanumeric         |
| date        | date    | Trading date, yyyy-mm-dd format  |
| open        | decimal | Opening price, two decimal places|
| high        | decimal | Highest price, two decimal places|
| low         | decimal | Lowest price, two decimal places |
| close       | decimal | Closing price, two decimal places|
| volume      | bigint  | Trading volume, integer          |
| turnover_rate| decimal| Turnover rate, four decimal places|
| prev_volume | bigint  | Previous trading volume, integer |
| ratio       | decimal | Ratio of volume change, four decimal places|

您可以根据您的具体需求和场景来调整表结构的细节，例如字段的长度、精度、约束等。您也可以使用一些数据库管理工具来帮助您创建和维护这个表，例如 MySQL、SQLite、MongoDB 等。


您好，如果您想再加一个前一天成交量和其比率两个字段，您可以这样做：

- **前一天成交量**：您可以从表中的volume字段获取，只需要将每一行的volume值与上一行的volume值对应起来。如果没有上一行，即第一行，那么前一天成交量可以设为0或者空值。
- **比率**：您可以根据以下公式计算出来，并添加到表中：

$$比率=\frac{当天成交量-前一天成交量}{前一天成交量}$$

例如，假设您有以下表：

| id | stock_code | date       | open  | high  | low   | close | volume |
|----|------------|------------|-------|-------|-------|-------|--------|
| 1  | AAPL       | 2022-12-31 | 182.8 | 184.9 | 181.1 | 182.3 | 123456 |
| 2  | AAPL       | 2023-01-03 | 181.5 | 183.7 | 180.2 | 181.9 | 234567 |
| 3  | AAPL       | 2023-01-04 | 182.1 | 184.2 | 181.6 | 183.4 | 345678 |

那么您可以计算出每天的前一天成交量和比率，并添加到表中，如下：

| id | stock_code | date       | open  | high  | low   | close | volume | prev_volume | ratio   |
|----|------------|------------|-------|-------|-------|-------|--------|-------------|---------|
| 1  | AAPL       | 2022-12-31 | 182.8 | 184.9 | 181.1 | 182.3 | 123456 |0            |-        |
| 2  | AAPL       | 2023-01-03 | 181.5 | 183.7 | 180.2 | 181.9 | 234567 |123456       |0.8998   |
| 3  | AAPL       | 2023-01-04 | 182.1 | 184.2 | 181.6 | 183.4 | 345678 |234567       |0.4737   |

注意，比率的单位是无量纲的，所以不需要乘以100。另外，第一行的比率可以设为0或者空值，因为没有前一天的数据。


您好，换手率是指一段时间内股票的成交量与流通股本的比值，反映了股票的活跃程度。换手率的计算公式是：

$$换手率=\frac{成交量}{流通股本}$$

如果您想在您提供的股票价格的表里加一个换手率字段，您需要有以下数据：

- **成交量**：您可以从表中的volume字段获取。
- **流通股本**：您需要从其他数据源获取，例如证券交易所或者财经网站。
- **换手率**：您可以根据上述公式计算出来，并添加到表中。

例如，假设您有以下表：

| id | stock_code | date       | open  | high  | low   | close | volume |
|----|------------|------------|-------|-------|-------|-------|--------|
| 1  | AAPL       | 2022-12-31 | 182.8 | 184.9 | 181.1 | 182.3 | 123456 |
| 2  | AAPL       | 2023-01-03 | 181.5 | 183.7 | 180.2 | 181.9 | 234567 |
| 3  | AAPL       | 2023-01-04 | 182.1 | 184.2 | 181.6 | 183.4 | 345678 |

假设AAPL的流通股本为16.7亿股，那么您可以计算出每天的换手率，并添加到表中，如下：

| id | stock_code | date       | open  | high  | low   | close | volume | turnover_rate |
|----|------------|------------|-------|-------|-------|-------|--------|---------------|
| 1  | AAPL       | 2022-12-31 | 182.8 | 184.9 | 181.1 | 182.3 | 123456 |0.0074%        |
| 2  | AAPL       | 2023-01-03 | 181.5 | 183.7 | 180.2 | 181.9 | 234567 |0.0140%        |
| 3  | AAPL       | 2023-01-04 | 182.1 | 184.2 | 181.6 | 183.4 | 345678 |0.0207%        |

注意，换手率的单位是百分比，所以需要乘以100。另外，不同的数据源可能有不同的定义和计算方法，所以您需要根据您的实际情况来调整。


您好，根据我的搜索结果，涨停价、跌停价的计算公式是：

- **涨停价**：是指股票在一个交易日内的最高允许价格，一般是前一天的收盘价乘以（1+涨幅比例），涨幅比例根据不同的股票类型和市场情况有所不同，一般是10%或者5%¹。涨停价的计算结果按照四舍五入原则取至价格最小变动单位²。
- **跌停价**：是指股票在一个交易日内的最低允许价格，一般是前一天的收盘价乘以（1-跌幅比例），跌幅比例与涨幅比例相同¹。跌停价的计算结果也按照四舍五入原则取至价格最小变动单位²。

如果您想在您提供的股票价格的表里再加两个字段：涨停价、跌停价，您需要有以下数据：

- **前一天的收盘价**：您可以从表中的close字段获取，只需要将每一行的close值与上一行的close值对应起来。如果没有上一行，即第一行，那么前一天的收盘价可以设为0或者空值。
- **涨幅比例**：您需要根据股票的类型和市场情况来确定，一般是10%或者5%，您可以从其他数据源获取，例如证券交易所或者财经网站。
- **涨停价、跌停价**：您可以根据上述公式计算出来，并添加到表中。

例如，假设您有以下表：

| id | stock_code | date       | open  | high  | low   | close | volume |
|----|------------|------------|-------|-------|-------|-------|--------|
| 1  | AAPL       | 2022-12-31 | 182.8 | 184.9 | 181.1 | 182.3 | 123456 |
| 2  | AAPL       | 2023-01-03 | 181.5 | 183.7 | 180.2 | 181.9 | 234567 |
| 3  | AAPL       | 2023-01-04 | 182.1 | 184.2 | 181.6 | 183.4 | 345678 |

假设AAPL的涨幅比例为10%，那么您可以计算出每天的涨停价和跌停价，并添加到表中，如下：

| id | stock_code | date       | open  | high  | low   | close | volume | prev_close| upper_limit| lower_limit|
|----|------------|------------|-------|-------|-------|-------|--------|-----------|------------|------------|
| 1  | AAPL       | 2022-12-31 | 182.8 | 184.9 | 181.1 | 182.3 | 123456 |-          |-           |-           |
| 2  | AAPL       | 2023-01-03 | 181.5 | 183.7 | 180.2 | 181.9 | 234567 |182.3      |200.5       |164.1       |
| 3  | AAPL       | 2023-01-04 | 182.1 | 184.2 | 181.6 | 183.4 | 345678 |181.9      |200.1       |163.7       |

注意，涨停价和跌停价的单位是美元，所以需要保留两位小数。另外，第一行的涨停价和跌停价可以设为0或者空值，因为没有前一天的数据。

¹: https://baike.baidu.com/item/涨停价/9264863
²: https://www.cnblogs.com/chuncn/archive/2009/07/09/1519551.html

Source: Conversation with Bing, 7/6/2023
(1) 涨停价_百度百科. https://baike.baidu.com/item/涨停价/9264863.
(2) 涨停价、跌停价、涨停板跌停价 - chuncn - 博客园. https://www.cnblogs.com/chuncn/archive/2009/07/09/1519551.html.
(3) 【思考14】量化交易回测中，关于涨跌停的处理方式 - 知乎. https://zhuanlan.zhihu.com/p/359675184.


您好，根据我的搜索结果，90%筹码、价格上限、价格下限、集中度的含义和计算方法是：

- **90%筹码**：是指流通股中有90%的持有者的成本在某一个区间¹。这个区间可以用价格上限和价格下限来表示。
- **价格上限**：是指90%筹码对应的最高价格，一般是用COST(95)函数来计算，表示95%获利盘的价格²。也就是有95%的筹码在该价格以下，为获利盘；5%的筹码在该价格以上，为套牢盘。
- **价格下限**：是指90%筹码对应的最低价格，一般是用COST(5)函数来计算，表示5%获利盘的价格²。也就是有5%的筹码在该价格以下，为获利盘；95%的筹码在该价格以上，为套牢盘。
- **集中度**：是指90%筹码的成本集中程度，一般用以下公式来计算：集中度=（价格上限-价格下限）/（价格上限+价格下限）*100³。集中度越低，表示筹码越集中；集中度越高，表示筹码越分散。

如果您想在您提供的股票价格的表里再加四个字段：90%筹码、价格上限、价格下限、集中度，您需要有以下数据：

- **前一天的收盘价**：您可以从表中的close字段获取，只需要将每一行的close值与上一行的close值对应起来。如果没有上一行，即第一行，那么前一天的收盘价可以设为0或者空值。
- **COST函数**：您需要使用COST函数来计算90%筹码对应的价格上限和价格下限。COST函数可以根据获利盘比例来计算相应的价格。例如，COST(95)表示95%获利盘对应的价格。
- **90%筹码、价格上限、价格下限、集中度**：您可以根据上述公式和数据计算出来，并添加到表中。

例如，假设您有以下表：

| id | stock_code | date       | open  | high  | low   | close | volume |
|----|------------|------------|-------|-------|-------|-------|--------|
| 1  | AAPL       | 2022-12-31 | 182.8 | 184.9 | 181.1 | 182.3 | 123456 |
| 2  | AAPL       | 2023-01-03 | 181.5 | 183.7 | 180.2 | 181.9 | 234567 |
| 3  | AAPL       | 2023-01-04 | 182.1 | 184.2 | 181.6 | 183.4 | 345678 |

假设AAPL的90%筹码对应的价格上限为200元，价格下限为160元，那么您可以计算出每天的90%筹码、价格上限、价格下限和集中度，并添加到表中，如下：

| id | stock_code | date       | open  | high  | low   | close | volume | prev_close| upper_limit| lower_limit| concentration|
|----|------------|------------|-------|-------|-------|-------|--------|-----------|------------|------------|--------------|
| 1  | AAPL       | 2022-12-31 | 182.8 | 184.9 | 181.1 | 182.3 | 123456 |-          |-           |-           |-             |
| 2  | AAPL       | 2023-01-03 | 181.5 | 183.7 | 180.2 | 181.9 | 234567 |182.3      |200         |160         |11.11%        |
| 3  | AAPL       | 2023-01-04 | 182.1 | 184.2 | 181.6 | 183.4 | 345678 |181.9      |200         |160         |11.11%        |

注意，90%筹码、价格上限、价格下限和集中度的单位都是美元，所以需要保留两位小数。另外，第一行的这些字段可以设为0或者空值，因为没有前一天的数据。

¹: https://zhuanlan.zhihu.com/p/149512961
²: https://zhuanlan.zhihu.com/p/594636490
³: https://www.zhihu.com/question/25330906

Source: Conversation with Bing, 7/6/2023
(1) 股票筹码90%集中度是怎么理解的？ - 知乎. https://www.zhihu.com/question/25330906.
(2) 股票筹码90%成本24一30元，筹码集中度12%是什么意思? - 知乎. https://zhuanlan.zhihu.com/p/149512961.
(3) 股票筹码峰怎么看?90%筹码集中度5%啥意思? - 知乎. https://zhuanlan.zhihu.com/p/594636490.

您好，根据我的搜索结果，如何使用rust编写COST函数的方法是：

- **定义函数**：您可以使用fn关键字来定义一个函数，给它一个合适的名称，例如cost。函数的参数需要指定类型，例如i32。如果函数有返回值，需要在箭头->后面指定返回类型，例如f64¹。
- **使用COST函数的公式**：您可以根据COST函数的含义和公式来编写函数体。COST函数表示X%获利盘对应的价格，也就是有X%的筹码在该价格以下，为获利盘；100-X%的筹码在该价格以上，为套牢盘²。COST函数的公式可以参考以下网址³。
- **返回计算结果**：您可以使用return关键字或者省略它，直接用最后一个表达式作为返回值。注意rust中的分号表示一个语句的结束，如果省略分号，表示一个表达式的值。

例如，您可以编写如下的rust代码来实现COST函数：

```rust
// 定义一个cost函数，接受一个i32类型的参数x，表示获利盘比例，返回一个f64类型的值，表示对应的价格
fn cost(x: i32) -> f64 {
    // 假设有一些数据表示股票价格和成交量的关系，这里用一个数组来模拟
    let data = [(100.0, 1000), (101.0, 800), (102.0, 600), (103.0, 400), (104.0, 200)];
    // 计算流通筹码总数，也就是成交量之和
    let total_chips = data.iter().map(|&(_, v)| v).sum::<i32>();
    // 计算X%获利盘对应的流通筹码数，也就是总数乘以X/100
    let target_chips = total_chips * x / 100;
    // 初始化一个变量来累计当前的流通筹码数
    let mut current_chips = 0;
    // 遍历数据数组，按照从高到低的价格顺序
    for &(price, volume) in data.iter().rev() {
        // 累加当前的流通筹码数
        current_chips += volume;
        // 如果当前的流通筹码数大于等于目标流通筹码数，说明找到了X%获利盘对应的价格
        if current_chips >= target_chips {
            // 返回这个价格作为函数的结果
            return price;
        }
    }
    // 如果没有找到对应的价格，说明数据有误或者参数有误，返回一个特殊值表示错误
    -1.0
}
```

¹: https://doc.rust-lang.org/book/ch03-03-how-functions-work.html
²: https://zhuanlan.zhihu.com/p/594636490
³: https://www.zhihu.com/question/25330906/answer/30715336

Source: Conversation with Bing, 7/6/2023
(1) Functions - The Rust Programming Language. https://doc.rust-lang.org/book/ch03-03-how-functions-work.html.
(2) Functions - The Rust Reference. https://doc.rust-lang.org/reference/items/functions.html.
(3) Functions - Rust By Example. https://doc.rust-lang.org/rust-by-example/fn.html.

## Fund 基金

您好，根据我的搜索结果，基金的数据主要包括以下几类：

- **基金基本信息**：包括基金的代码、名称、类型、成立日期、规模、管理人、托管人等。
- **基金净值数据**：包括基金的单位净值、累计净值、日增长率、年化收益率等。
- **基金分红数据**：包括基金的分红日期、分红金额、分红方式等。
- **基金持仓数据**：包括基金的持有股票、债券、现金等资产的代码、名称、市值、占比等。
- **基金业绩数据**：包括基金的收益率、风险指标、评级等。

如果您想设计一个存储基金价格的表，您可以参考以下的一个例子：

| id | fund_code | date       | unit_nav | accum_nav | daily_return |
|----|-----------|------------|----------|-----------|--------------|
| 1  | 000001    | 2022-12-31 | 1.2345   | 2.3456    | 0.0123       |
| 2  | 000001    | 2023-01-03 | 1.2456   | 2.3567    | 0.0090       |
| 3  | 000001    | 2023-01-04 | 1.2567   | 2.3678    | 0.0089       |
| ...| ...       | ...        | ...      | ...       | ...          |

这个表的含义是：

- id：每条记录的唯一标识符，可以是自增的整数或者其他类型。
- fund_code：基金的代码，可以是数字或者字母，根据不同的市场和交易所有不同的规则。
- date：基金的净值日期，可以是年月日的格式，或者其他格式，如时间戳等。
- unit_nav：基金的单位净值，即每份基金的净资产价值。
- accum_nav：基金的累计净值，即每份基金的累计净资产价值，包括分红和复投。
- daily_return：基金的日增长率，即当天单位净值相对于前一天单位净值的变化百分比。

这个表只是一个简单的示例，您可以根据您的具体需求和场景来设计更复杂或者更简单的表。您也可以使用一些数据库管理工具来帮助您创建和维护这个表，例如 MySQL、SQLite、MongoDB 等。您也可以参考一些专业的金融数据平台，例如中国经济金融研究数据库（CSMAR）¹，它提供了丰富和全面的基金数据。

¹: http://www.csmar.com/channels/31.html

Source: Conversation with Bing, 7/6/2023
(1) 获取基金行情数据 - 知乎 - 知乎专栏. https://zhuanlan.zhihu.com/p/439266861.
(2) UNEP：关于COP27损失和损害基金您需要了解的点｜绿会国际讯. https://baijiahao.baidu.com/s?id=1750884079536467548.
(3) 中国经济金融研究数据库（CSMAR）-深圳希施玛数据科技有限公司. http://www.csmar.com/channels/31.html.



您好，根据我的搜索结果，有一些网站或者平台提供了开放的api获取基金数据，例如：

- **AKShare**¹：是一个基于Python的开源金融数据接口库，提供了丰富和全面的中国股票，基金，债券和宏观经济数据。您可以通过pip安装akshare库，然后使用相应的函数来获取基金数据，例如：

```python
import akshare as ak
fund_em_open_fund_daily_df = ak.fund_em_open_fund_daily() # 获取公募开放式基金数据
fund_em_open_fund_daily_df
```

- **Tushare**²：是一个提供金融数据分析服务的平台，提供了中国股票，基金，期货等市场的历史和实时数据。您可以通过注册账号并获取token来使用Tushare的api，例如：

```python
import tushare as ts
ts.set_token('your token') # 设置token
pro = ts.pro_api() # 初始化pro接口
df = pro.fund_nav(ts_code='150018.SZ') # 获取基金净值数据
df
```

- **天行数据**³：是一个开发者api数据平台，提供了多种类型的数据接口，包括基金数据。您可以通过注册账号并获取key来使用天行数据的api，例如：

```python
import requests
url = 'http://api.tianapi.com/txapi/fund/index' # 基金接口地址
params = {
    'key': 'your key', # 请求密钥
    'code': '000001' # 基金代码
}
response = requests.get(url, params=params) # 发送get请求
response.json() # 查看返回的json数据
```

以上只是一些示例，您还可以参考其他的网站或者平台，如API Shop、Investpy、Yahooquery等，它们也提供了一些获取基金数据的api。您可以根据您的需求和喜好选择合适的api来使用。

¹: https://github.com/akfamily/akshare
²: https://tushare.pro/
³: https://www.tianapi.com/apiview/97
: https://www.apishop.net/
: https://investpy.readthedocs.io/
: https://yahooquery.dpguthrie.com/

Source: Conversation with Bing, 7/6/2023
(1) 使用Python轻松获取股票&基金数据 - 知乎 - 知乎专栏. https://zhuanlan.zhihu.com/p/393083394.
(2) 分享互联网公开api接口网站——各式各样的接口都有 - 知乎. https://zhuanlan.zhihu.com/p/339188408.
(3) 全网最全的API接口合集-整理 - 知乎 - 知乎专栏. https://zhuanlan.zhihu.com/p/314212747.

## Web scraping

To scrape dynamic product list pages, you can use a headless browser like **Puppeteer** or **Selenium**. These tools can simulate user interactions with the page and execute JavaScript code to generate the dynamic content. You can also use **Scrapy-Splash**, which is a lightweight Python library that uses Splash to render JavaScript and allows you to scrape dynamic web pages.

Here are some resources that can help you get started:
- [How to Scrape Product List and Detail Pages with GrabzIt](https://grabz.it/scraper/documentation/tutorials/scraping-list-detail-pages/)
- [Dynamic Web Pages Scraping with Python: Guide to Scrape All ... - ZenRows](https://www.zenrows.com/blog/dynamic-web-pages-scraping-python)
- [Scraping Dynamic eCommerce Product Pages | by Lily Wu | Medium](https://medium.com/@didsomeonesaydata/scraping-dynamic-ecommerce-product-pages-195d13ab3ef8)

I hope this helps! Let me know if you have any other questions.

Source: Conversation with Bing, 7/10/2023
(1) How to Scrape Product List and Detail Pages with GrabzIt. https://grabz.it/scraper/documentation/tutorials/scraping-list-detail-pages/.
(2) Dynamic Web Pages Scraping with Python: Guide to Scrape All ... - ZenRows. https://www.zenrows.com/blog/dynamic-web-pages-scraping-python.
(3) Scraping Dynamic eCommerce Product Pages | by Lily Wu | Medium. https://medium.com/@didsomeonesaydata/scraping-dynamic-ecommerce-product-pages-195d13ab3ef8.
(4) How to scrape dynamic content from a website? - Stack Overflow. https://stackoverflow.com/questions/55709463/how-to-scrape-dynamic-content-from-a-website.


Sure! Here's a Python code snippet that uses **Selenium** to scrape dynamic product list pages:

```python
from selenium import webdriver
from selenium.webdriver.common.keys import Keys

driver = webdriver.Chrome()
driver.get("https://www.example.com")

# Click on the page number you want to scrape
page_number = driver.find_element_by_xpath("//a[@href='/page2']")
page_number.click()

# Scrape the data from the page
data = driver.find_elements_by_xpath("//div[@class='product']")

for item in data:
    print(item.text)

driver.quit()
```

You can modify this code snippet to suit your needs. Let me know if you have any other questions.


Yes! There is a Rust library called **thirtyfour** that is similar to Selenium. It is a Selenium / WebDriver library for Rust that supports the full W3C WebDriver spec and is used for automated website UI testing. It has been tested with Chrome and Firefox although any W3C-compatible WebDriver should work. 

Here are some resources that can help you get started:
- [thirtyfour - Rust - Docs.rs](https://docs.rs/thirtyfour/latest/thirtyfour/)
- [Using Selenium with Rust - DEV Community](https://dev.to/stevepryde/using-selenium-with-rust-aca)
- [GitHub - stevepryde/thirtyfour: Selenium WebDriver client for Rust, for ...](https://github.com/stevepryde/thirtyfour)

I hope this helps! Let me know if you have any other questions.

Source: Conversation with Bing, 7/10/2023
(1) thirtyfour - Rust - Docs.rs. https://docs.rs/thirtyfour/latest/thirtyfour/.
(2) Using Selenium with Rust - DEV Community. https://dev.to/stevepryde/using-selenium-with-rust-aca.
(3) GitHub - stevepryde/thirtyfour: Selenium WebDriver client for Rust, for .... https://github.com/stevepryde/thirtyfour.

Yes! You can use **Puppeteer** to scrape dynamic web pages. Puppeteer runs a headless browser that you can use to scrape a webpage for its content using HTML DOM selectors. You can also configure your code to randomly change the browser fingerprint and IP address.

Here are some resources that can help you get started:
- [How to do Web Scraping with Puppeteer and NodeJS - ZenRows](https://www.zenrows.com/blog/puppeteer-web-scraping)
- [How To Scrape a Website Using Node.js and Puppeteer](https://www.digitalocean.com/community/tutorials/how-to-scrape-a-website-using-node-js-and-puppeteer)
- [Web Scraping with Puppeteer - 2023 Guide - Bright Data](https://brightdata.com/blog/how-tos/web-scraping-puppeteer)

I hope this helps! Let me know if you have any other questions.

Source: Conversation with Bing, 7/10/2023
(1) How to do Web Scraping with Puppeteer and NodeJS - ZenRows. https://www.zenrows.com/blog/puppeteer-web-scraping.
(2) How to scrape a dynamic webpage using puppeteer-core in a deployed .... https://stackoverflow.com/questions/53966973/how-to-scrape-a-dynamic-webpage-using-puppeteer-core-in-a-deployed-application.
(3) How To Scrape a Website Using Node.js and Puppeteer. https://www.digitalocean.com/community/tutorials/how-to-scrape-a-website-using-node-js-and-puppeteer.
(4) Web Scraping with Puppeteer - 2023 Guide - Bright Data. https://brightdata.com/blog/how-tos/web-scraping-puppeteer.

Sure! Here's a JavaScript code snippet that uses **Puppeteer** to scrape dynamic web pages:

```javascript
const puppeteer = require('puppeteer');

(async () => {
  const browser = await puppeteer.launch();
  const page = await browser.newPage();
  await page.goto('https://www.example.com');
  const data = await page.evaluate(() => {
    const items = document.querySelectorAll('.item');
    return Array.from(items).map(item => item.textContent);
  });
  console.log(data);
  await browser.close();
})();
```

You can modify this code snippet to suit your needs. Let me know if you have any other questions.

Yes, you can limit the URLs only from `example.com` by checking if each URL found during crawling starts with `https://www.example.com`. Here is an example of how to modify the previous code to do it:

```rust
use scraper::{Html, Selector};
use reqwest::blocking::Client;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let mut visited = std::collections::HashSet::new();
    let mut queue = vec!["https://www.example.com"];
    while let Some(url) = queue.pop() {
        if visited.contains(url) {
            continue;
        }
        visited.insert(url.to_owned());
        let res = client.get(url).send()?;
        let body = res.text()?;
        let document = Html::parse_document(&body);
        let selector = Selector::parse("a").unwrap();
        for link in document.select(&selector) {
            if let Some(href) = link.value().attr("href") {
                if href.starts_with("https://www.example.com") && !visited.contains(href) {
                    queue.push(href);
                }
            }
        }
    }
    for url in visited {
        println!("{}", url);
    }
    Ok(())
}
```

This code checks if each URL found during crawling starts with `"https://www.example.com"` before adding it to the queue of URLs to visit next. The code repeats this process until there are no more URLs left in the queue.

You can find more information about these crates in their respective documentation:

- [reqwest](https://docs.rs/reqwest)
- [scraper](https://docs.rs/scraper)

You're right that `reqwest` and `scraper` cannot scrape dynamic content generated by JavaScript. In that case, you can use a headless browser like [Puppeteer](https://pptr.dev/) or [Playwright](https://playwright.dev/) to scrape the content.

Here is an example of how you can use Puppeteer to scrape dynamic content:

```rust
use std::sync::Arc;
use tokio::runtime::Runtime;
use tauri::{plugin::Plugin, Invoke};

mod scraper;

fn main() {
    let plugin = Plugin::new(|mut app| {
        app.invoke_handler(|_webview, arg| {
            let url = arg
                .get("url")
                .and_then(|u| u.as_str())
                .ok_or_else(|| "missing url argument")?;
            let rt = Runtime::new().unwrap();
            let content = rt.block_on(scraper::scrape(url))?;
            Ok(content)
        })
    });
    tauri::Builder::default()
        .plugin(plugin)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

This code uses the `tokio` crate to run an asynchronous function that uses Puppeteer to scrape the specified URL. The `scraper` module contains the Rust code for scraping web content using Puppeteer.

You can find more information about using Puppeteer with Rust in the [Puppeteer Rust documentation](https://docs.rs/puppeteer/).

I hope this helps!

## 开放数据
1. [基金历史净值明细](http://fundf10.eastmoney.com/jjjz_006718.html)


## web scraping with javascript + tauri

```js

use tauri::api::webview::{WebviewAttributes, InvokePayload};
use tauri::Manager;
use js_sys::Reflect;
use wasm_bindgen::JsCast;

fn main() {
  let mut webview_attributes = WebviewAttributes::new("https://tauri.studio").unwrap();
  webview_attributes = webview_attributes.initialization_script("
    const cheerio = require('cheerio');
    const html = document.documentElement.outerHTML;
    const $ = cheerio.load(html);
    $('h2.title').text('Hello there!');
    $('h2').addClass('welcome');
    console.log($.html());
  ");
  Manager::default().create_window(webview_attributes).unwrap();
}

```

## how to get all the plugins information in a tauri application?

There is no built-in way to get all the plugins information in a Tauri application, but you can write a custom function to do that. You can use the App::plugin_store method to get a reference to the plugin store, which is a HashMap that maps plugin names to plugin instances. You can then iterate over the plugin store and get the name and other information of each plugin. For example:

```rust
// src-tauri/src/main.rs
use tauri::{AppBuilder, Manager, plugin::Plugin};

fn main() {
  tauri::AppBuilder::new()
    .plugin(CustomPlugin) // register the custom plugin
    .setup(|app| {
      // get all the plugins information
      let plugins = get_plugins_info(app);
      // print the plugins information
      println!("{:?}", plugins);
      Ok(())
    })
    .build()
    .run();
}

struct CustomPlugin;

impl Plugin for CustomPlugin {
  fn name(&self) -> &'static str {
    "custom-plugin"
  }

  fn extend_api(&mut self, webview: &mut WebView<'_>, payload: &str) {
    // ...
  }
}

// a custom function to get all the plugins information
fn get_plugins_info(app: &App) -> Vec<PluginInfo> {
  // get a reference to the plugin store
  let plugin_store = app.plugin_store();
  // create an empty vector to store the plugins information
  let mut plugins = Vec::new();
  // iterate over the plugin store
  for (name, plugin) in plugin_store.iter() {
    // create a PluginInfo struct with the name and other information of the plugin
    let info = PluginInfo {
      name: name.to_string(),
      // add other information as needed, such as version, description, etc.
      // ...
    };
    // push the PluginInfo struct to the vector
    plugins.push(info);
  }
  // return the vector of plugins information
  plugins
}

// a custom struct to store the plugin information
#[derive(Debug)]
struct PluginInfo {
  name: String,
  // add other fields as needed, such as version, description, etc.
  // ...
}
```

This way, you can get all the plugins information in a Tauri application. I hope this helps.
