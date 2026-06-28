// TODO: 定义一个新的 `Order` 类型。
//   它应该跟踪三条信息：`product_name`、`quantity` 和 `unit_price`。
//   产品名称不能为空，且不能超过 300 字节。
//   数量必须严格大于零。
//   单价以分为单位，必须严格大于零。
//   Order 必须包含一个名为 `total` 的方法，返回订单的总价格。
//   Order 必须为每个字段提供 setter 和 getter 方法。
//
// 这次测试位于不同的位置——在 `tests` 文件夹中。
// `tests` 文件夹是 `cargo` 的一个特殊位置。它在这里查找**集成测试**。
// 这里的集成有一个非常具体的含义：它们测试你项目的**公共 API**。
// 你需要注意你的类型和方法的可见性；集成测试
// 无法访问私有或 `pub(crate)` 的项。
pub struct Order{
    product_name: String,
    quantity: u32,
    unit_price: u32,
}

impl Order {
    pub fn new(product_name: String, quantity:u32, unit_price:u32)-> Self{
        if product_name.len()==0{
            panic!("Product name cannot be empty");
        }
        if product_name.len()>300{
            panic!("Product name cannot be longer than 300 bytes");
        }
        if quantity<=0{
            panic!("Quantity must be greater than 0");
        }
        if unit_price<=0{
            panic!("Unit price must be greater than 0");
        }
        Self{
            product_name,
            quantity,
            unit_price,
        }
    }

    pub fn product_name(&self)->&str{
        &self.product_name
    }

    pub fn quantity(&self)->&u32{
        &self.quantity
    }

    pub fn unit_price(&self)->&u32{
        &self.unit_price
    }

    pub fn set_product_name(&mut self,product_name: String){
        if product_name.len()==0{
            panic!("Product name cannot be empty");
        }
        if product_name.len()>300{
            panic!("Product name cannot be longer than 300 bytes");
        }
        self.product_name=product_name;
    }

    pub fn set_quantity(&mut self,quantity:u32){
        if quantity<=0{
            panic!("Quantity must be greater than 0");
        }
        self.quantity=quantity;
    }

    pub fn set_unit_price(&mut self,unit_price:u32){
        if unit_price<=0{
            panic!("Unit price must be greater than 0");
        }
        self.unit_price=unit_price;
    }

    pub fn total(&self)->u32{
        self.quantity*self.unit_price
    }
}
