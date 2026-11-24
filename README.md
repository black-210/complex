# Complex

مكتبة Rust خفيفة للعمليات على الأعداد المركبة ومعالجة إشارات الاتصالات الرقمية باستخدام خوارزميات وصفائف مصفوفات.

## نظرة عامة

يوفر المشروع نوعًا مستقلًا باسم `Complex` لتمثيل عدد مركب من جزأين:

- `re`: الجزء الحقيقي.
- `im`: الجزء التخيلي.

تم بناء المكتبة لتكون قابلة للاستخدام كصفوف حسابية، كما توفر الدوال الرياضية ودوال معالجة الإشارات الأساسية.

## المميزات

- إنشاء الأعداد المركبة ومعالجة الجزأين الحقيقي والتخيلي.
- الجمع والضرب والقسمة ودعم المعاملات الحسابية في Rust.
- حساب المرافق `conj` والمقدار `abs` والطور `arg`.
- حساب الدالة الأسية وتحويل الإحداثيات القطبية باستخدام `from_polar`.
- دعم تجريبي لتعليمات SIMD على معمارية `x86_64` عبر `exp_simd`.
- استخدام مصفوفات `nalgebra` ثنائية الأبعاد والمتجهات للإشارات المستقبلة.
- استعادة الإشارة الأصلية من مصفوفة القناة والإشارة المستقبلة.
- تحويل إشارة بطول ثمانية عينات عبر `hardwarehackphy`.
- توفير خطأ واضح لحالة المصفوفة غير القابلة للعكس.
- اختبارات وحدة للعمليات الأساسية والدوال المساعدة والأسية.

## التقنيات المستخدمة

- **Rust Edition 2024**
- **nalgebra 0.35.0**
- مصفوفة `SMatrix` والمتجه `Vector2`
- تعليمات SSE عند توفرها على معمارية `x86_64`
- اختبارات Rust المدمجة باستخدام `#[cfg(test)]`

## مثال للاستخدام

```rust
use complex::Complex;

let z1 = Complex::new(1.0, 2.0);
let z2 = Complex::new(3.0, 4.0);
let sum = z1 + z2;
let product = z1 * z2;
let quotient = z1 / z2;
let magnitude = Complex::abs(z1);

println!("{:?}", sum);
println!("{:?}", product);
println!("{:?}", quotient);
println!("Magnitude: {}", magnitude);
```

## استعادة الإشارة الأصلية

تعيد الدالة `recover_original_signal` الإشارة الأصلية من مصفوفة قناة بحجم `2 × 2` ومتجه استقبال بحجم عنصرين:

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

يرجع الخطأ `HardwareMathError::UninvertibleChannelMatrix` إذا كان محدد مصفوفة القناة يساوي صفرًا.

## التحويل التجريبي للإشارة

تتطلب الدالة `hardwarehackphy` مقطعًا من ثمانية أعداد مركبة وتعيد مصفوفة من ثمانية مخرجات:

```rust
let input = [Complex::new(1.0, 0.0); 8];
let output = Complex::hardwarehackphy(&input);
```

## إضافات قيم مرجعية وحسابات تجريبية

توفر المكتبة مجموعتين ثابتتين وواجهتين حسابيتين تجريبيتين:

- `golden_sweep_probe` تعيد 17 عددًا مركبًا ثابتًا.
- `chaotic_golden_inverse_manifold` تعيد 31 عددًا مركبًا ثابتًا.
- `hw_fragility(a, b, c)` تحسب قيمة مركبة تجريبية من ثلاثة مدخلات.
- `jump(a, b, c, d, e, f, g)` تحسب قيمة مركبة تجريبية من سبعة مدخلات.

```rust
let probe = Complex::golden_sweep_probe();
let manifold = Complex::chaotic_golden_inverse_manifold();

let a = Complex::new(0.7, -0.2);
let b = Complex::new(0.4, 0.9);
let c = Complex::new(-0.3, 0.5);
let d = Complex::new(1.2, 0.8);
let e = Complex::new(0.6, 1.1);
let f = Complex::new(-0.5, 0.7);
let g = Complex::new(0.9, -0.4);

let fragility = Complex::hw_fragility(a, b, c);
let jump_value = Complex::jump(a, b, c, d, e, f, g);
```

تُعد الصيغ والقيم المرجعية تجريبية ويُنصح بالتحقق منها قبل الاعتماد عليها في تطبيقات الإنتاج.

## التشغيل والاختبار

```bash
cargo test
cargo check
cargo fmt --check
cargo clippy -- -D warnings
```

## توافق الأسماء السابقة

للحفاظ على توافق المصدر، ما زالت الأسماء السابقة متاحة بصيغة مهملة:

- `recover_orignal_signal`
- `UninvertibleCannelMatrix`
- `golden_sweeq_probe`
- `chatotic_golden_inverse_manifold`
