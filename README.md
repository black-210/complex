# Complex

مكتبة Rust للأعداد المركبة ومعالجة الإشارات الرقمية، مبنية لتقديم واجهة حسابية واضحة، ودوال رياضية عملية، ودعم متميز للمصفوفات والمتجهات عبر مكتبة `nalgebra`.

## نظرة عامة

يمثل المشروع العدد المركب من جزأين:

- `re`: الجزء الحقيقي.
- `im`: الجزء التخيلي.

توفر المكتبة عمليات رياضية سهلة، ومعاملات Rust القياسية، وأدوات مفيدة لتجارب الاتصالات ومعالجة الإشارات.

## المميزات الأساسية

- إنشاء الأعداد المركبة باستخدام `Complex::new`.
- الجمع والطرح والضرب والقسمة عبر الدوال والمعاملات `+` و`-` و`*` و`/`.
- حساب المرافق بالمركب باستخدام `conjugate` أو `conj`.
- حساب المقدار باستخدام `magnitude` أو `abs`.
- حساب الطور باستخدام `argument` أو `arg`.
- حساب الشكل الأسّي للأعداد المركبة باستخدام `exp`.
- إنشاء عدد مركب من الإحداثيات القطبية باستخدام `from_polar`.
- الوصول إلى دوال SIMD محسنة على معمارية `x86_64` باستخدام `exp_simd`.
- العمل مع مصفوفات `SMatrix` ثنائية الأبعاد ومتجهات `Vector2`.
- استعادة الإشارات الأصلية من مصفوفة قناة.
- معالجة مقاطع مكونة من ثمانية عينات مركبة باستخدام `hardwarehackphy`.
- تصفية وقياس القيم المرجعية الثابتة والمولدة للحسابات والتجارب.
- دعم أسماء حديثة وطرق عمل جديدة مع الحفاظ على توافق الأسماء القديمة.
- اختبارات وحدة تغطي العمليات الأساسية والاستعادة والمعالجة والمصفوفات والأسية.

## التقنيات المستخدمة

- Rust Edition 2024.
- مكتبة `nalgebra` للمصفوفات والمتجهات والأعداد المركبة.
- SSE على معمارية `x86_64`.
- إمكانات واضحة للتحكم بحالة الحساب في واجهة معالجة الإشارات.
- اختبارات Rust المدمجة باستخدام `#[cfg(test)]`.
- صيغ حسابية رقمية قابلة لإعادة الاستخدام.

## بنية المشروع

```text
complex/
├── Cargo.toml
├── Cargo.lock
└── src/
    └── lib.rs
```

`Cargo.toml` يحدد اسم الحزمة والإصدار والتبعيات، بينما يحتوي `src/lib.rs` على النوع الرياضي والدوال والاختبارات.

## البدء السريع

```rust
use complex::Complex;

let z1 = Complex::new(1.0, 2.0);
let z2 = Complex::new(3.0, 4.0);

let sum = z1 + z2;
let difference = z1 - z2;
let product = z1 * z2;
let quotient = z1 / z2;

println!("المجموع: {:?}", sum);
println!("الفرق: {:?}", difference);
println!("الضرب: {:?}", product);
println!("القسمة: {:?}", quotient);
```

## العمليات الرياضية

```rust
let z = Complex::new(3.0, 4.0);

let conjugate = Complex::conjugate(z);
let magnitude = Complex::magnitude(z);
let argument = Complex::argument(z);
let exponential = Complex::exp(z);
let polar = Complex::from_polar(5.0, 1.57079632679);
```

كل الدوال تعمل مباشرةً مع القيم المركبة الممثلة بالجزأين الحقيقي والتخيلي، وتوفر واجهة ثابتة بين الاستدعاءات المباشرة والمعاملات الحسابية.

## استعادة الإشارة الأصلية

تستعيد الدالة `recover_original_signal` الإشارة الأصلية من مصفوفة قناة بحجم `2 × 2` ومتجه استقبال بحجم عنصرين:

```rust
use complex::Complex;
use nalgebra::{SMatrix, Vector2};

let channel = SMatrix::<Complex, 2, 2>::new(
    Complex::new(1.0, 0.0),
    Complex::new(0.0, 0.0),
    Complex::new(0.0, 0.0),
    Complex::new(1.0, 0.0),
);

let received = Vector2::new(
    Complex::new(1.0, 2.0),
    Complex::new(3.0, -1.0),
);

let result = Complex::recover_original_signal(&channel, &received);
```

تعيد الدالة متجهًا من قيم `Complex` عند نجاح الحساب، وتستخدم نوعًا واضحًا من نوع `HardwareMathError` للتحكم بحالة المصفوفة.

## معالجة المقاطع ذات الثمانية عينات

تعالج الدالة `hardwarehackphy` مقطعًا ثابتًا من ثمانية أعداد مركبة وتعيد مصفوفة من ثمانية مخرجات:

```rust
let input = [Complex::new(1.0, 0.0); 8];
let output = Complex::hardwarehackphy(&input);

assert_eq!(output.len(), 8);
```

تستخدم الدالة جمعًا وطرحًا وضربًا مركبًا ووزنات حسابية ثابتة، مما يجعلها مناسبة لبناء خطوط معالجة إشارات متتابعة.

## تحديث جديد — أدوات الحساب والاختبار المتقدمة

أضيفت مجموعة أحدث من المزايا لإثراء التحليل وإعادة إنتاج التجارب:

### مجموعات القيم المرجعية الثابتة

#### `golden_sweep_probe`

تعيد مصفوفة تحتوي على 17 عددًا مركبًا ثابتًا ومنظمًا، ويمكن استخدامها كقيمة probe موحدة في الاختبارات والمقارنات:

```rust
let probe = Complex::golden_sweep_probe();

assert_eq!(probe.len(), 17);

for value in probe {
    println!("{:?}", value);
}
```

#### `chaotic_golden_inverse_manifold`

توفر مصفوفة من 31 عددًا مركبًا ثابتًا لتوسيع نطاق حالات الاختبار والقيم المرجعية:

```rust
let manifold = Complex::chaotic_golden_inverse_manifold();

assert_eq!(manifold.len(), 31);
println!("{:?}", manifold[0]);
println!("{:?}", manifold[30]);
```

### مولدات الاختبار متعددة النتائج

#### `chaotic_hardware_probe`

تنشئ 17 عددًا مركبًا من قيمة بذرة `u64`، وتجمع الزوايا وفقًا للفهرس لإنتاج قيم سين/كوس متتابعة:

```rust
let seed = 2024_u64;
let probe = Complex::chaotic_hardware_probe(seed);

assert_eq!(probe.len(), 17);
```

#### `chaotic_hardware`

تنشئ 17 قيمة محددة وعائدًا مباشرًا بناءً على عامل عددي `f64`، مع مجموعة قوالب دورية تغطي ستة أنماط:

```rust
let values = Complex::chaotic_hardware(1.618);
assert_eq!(values.len(), 17);
```

تدعم هذه الدالة قيمًا أدق وإجراءات اختبار قابلة لإعادة التنفيذ، وتعمل أيضًا كمدخلات مناسبة للتجارب التي تحتاج مجموعات مركبة منتظمة.

### حسابات الريادة والتبديل

#### `hw_fragility`

تحسب قيمة مركبة من ثلاثة أعداد مركبة `a` و`b` و`c`، وتجمع معلومات الجزأين الحقيقي والتخيلي في صيغة حسابية مرنة:

```rust
let a = Complex::new(0.7, -0.2);
let b = Complex::new(0.4, 0.9);
let c = Complex::new(-0.3, 0.5);

let result = Complex::hw_fragility(a, b, c);
```

#### `jump`

تحسب قيمة مركبة من سبعة مدخلات، وتسمح بتحليل الحالات المتغيرة عبر قيم `a` و`b` و`c` و`d` و`e` و`f` و`g`:

```rust
let result = Complex::jump(a, b, c, d, e, f, g);
```

### الميزة الجديدة `bump`

توفر الدالة `bump` حسابًا مركبًا موسعًا من تسعة مدخلات، وتعيد ناتجًا مركبًا يجمع القيم الحقيقية والتخيلية في طبقات حسابية متعددة:

```rust
let result = Complex::bump(a, b, h, i, k, l, o, z, m);
```

تتميز هذه الدالة بمرونة المدخلات وقدرتها على دمج معاملات من ثلاثة أنماط مختلفة داخل نتيجة واحدة، ويمكن توظيفها في تجارب الإشارات والقياس والحسابات متعددة القيم.

### ميزة تبديل الأجزاء `permute`

تستخدم الدالة `permute` أسلوبًا مباشرًا لتبديل الجزء الحقيقي والجزء التخيلي:

```rust
let z = Complex::new(3.0, 4.0);
let permuted = z.permute();

assert_eq!(permuted, Complex::new(4.0, 3.0));
```

هذه الميزة مفيدة في اختبارات إعادة ترتيب الإحداثيات، وتبسيط المحاولات الحسابية، وتحويل تمثيل القيمة المركبة بسرعة.

## تشغيل المشروع

شغّل اختبارات المشروع:

```bash
cargo test
```

افحص صحة المشروع:

```bash
cargo check
```

نسق الملفات:

```bash
cargo fmt
```

شغّل فاحص Clippy:

```bash
cargo clippy
```

## توافق الأسماء

المشروع يحافظ على سهولة الانتقال بين واجهته القديمة والحديثة، إذ توفر الأسماء التاريخية إمكانات التوافق:

- `recover_original_signal` و`recover_orignal_signal`.
- `UninvertibleChannelMatrix` و`UninvertibleCannelMatrix`.
- `golden_sweep_probe` و`golden_sweeq_probe`.
- `chaotic_golden_inverse_manifold` و`chatotic_golden_inverse_manifold`.

يمكن للمشاريع الجديدة استخدام الأسماء الحديثة مباشرةً والاستفادة من واجهة أوضح ومتسقة.
