use dioxus::events::MouseEvent;
use dioxus::prelude::*;
use mysql::*;
use mysql::prelude::*;

#[derive(Clone, Default)]
struct TradeInfo {
    id: i32,
    source: String,
    dest: String,
    value: i32,
}

fn main() {
    dioxus::desktop::launch(App);
}

// 这里我们就将上面所编写的 Props 绑入了组件中
fn App(cx: Scope) -> Element {

    // 将trade info list 绑定在use_state中
    let trade_info_list = use_state(
        &cx,
        || vec![TradeInfo::default()],
    );

    // conn the db
    let mut conn = getConn();

    cx.render(rsx! (

        div {
            // 将trade_info_list 渲染为一堆 `li` 标签：
            trade_info_list.iter().map(|trade_info| rsx! (
                li {"交易单号：{trade_info.id}"}
                li {"发起账号：{trade_info.source}"}
                li {"目的账号：{trade_info.dest}"}
                li {"交易金额：{trade_info.value}"}
                hr {}
            ))

            button { onclick: move |_| {
                let ret = conn.query_map(
                    "select * from tradeinfos",
                    |(id, source, dest, value) | TradeInfo {
                        id,
                        source,
                        dest,
                        value,
                    },
                ).expect("Query failed");

                trade_info_list.set(ret);

            }, "查找交易信息" }

            button { onclick: move |_| {
                // 删除交易信息
                // run the sql delete
                "delete from tradeinfos".run(getConn()).expect("delete failed");
            },
            "删除交易信息"}

        }

    ))
}

// 返回一个数据库链接 conn
fn getConn() -> PooledConn {
    let url = "mysql://root:password@localhost:3306/dbname";
    let pool = Pool::new(url).unwrap(); // 获取连接池
    pool.get_conn().unwrap()// 获取链接
}
