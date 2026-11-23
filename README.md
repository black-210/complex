# Complex

مكتبة Rust خفيفة للعمليات على الأعداد المركبة، ومعالجة إشارات الاتصال الرقمية باستخدام خوارزميات وصفائف مصفوفية من خلال مكتبة `nalgebra`.

## نظرة عامة

يوفر المشروع نوعًا مستقلًا باسم `Complex` لتمثيل عدد مركب من جزأين:

- `re`: الجزء الحقيقي.
- `im`: الجزء التخيلي.

تم بناء المكتبة لتكون قابلة للاستخدام كصفوف حسابية، كما توفر دوال خاصة بالحسابات الرياضية ومعالجة الإشارات في أنظمة الاتصال اللاسلكية.

## المميزات

- إنشاء الأعداد المركبة ومعالجة جزئيها الحقيقي والتخيلي.
- الجمع والضرب والقسمة، ودعم الجمع والضرب والقسمة عبر معاملات Rust.
- الضرب والقسمة باستخدام صيغ الأعداد المركبة.
- حساب المرافق `conj`.
- حساب المقدار `abs`.
- حساب الطور `arg`.
- تحويل الأعداد المركبة إلى الصيغة الأسية `exp`.
- إنشاء عدد مركب من الإحداثيات القطبية باستخدام `from_polar`.
- دعم عمليات SIMD تجريبية على معمارية `x86_64` عبر `exp_simd`.
- استخدام مصفوفات `nalgebra` ثنائية الأبعاد وتمثيلات متجهية للإشارات المستقبلة.
- استعادة الإشارة الأصلية من مصفوفة القناة والإشارة المستقبلة.
- تحويل إشارة بطول ثمانية عينات عبر `hardwarehackphy`.
- معاملات خطأ واضحة لحالة المصفوفة غير القابلة للعكس.
- اختبارات وحدة تغطي العمليات الأساسية والدوال المساعدة والدوال الأسية.

## التقنيات المستخدمة

- **Rust Edition 2024**
- **nalgebra 0.35.0**
- فئات `SMatrix` و `Vector2`
- تعليمات SSE عبر `_mm_set1_pd` و `_mm_mul_pd` و `_mm_cvtsd_f64`
- اختبارات Rust المدمجة باستخدام وحدة `#[cfg(test)]`

## بنية المشروع

```text
complex/
├── Cargo.toml
└── src/
    └── lib.rs
```

- `Cargo.toml`: إعدادات الحزمة واعتماد مكتبة `nalgebra`.
- `src/lib.rs`: تعريف `Complex` والعمليات الرياضية ودوال معالجة الإشارة والاختبارات.

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

يمكن أيضًا استخدام الدوال المسماة مباشرةً:

```rust
let sum = Complex::add(z1, z2);
let conjugate = Complex::conj(z1);
let exponential = Complex::exp(z1);
let polar = Complex::from_polar(5.0, 0.785398);
```

## استعادة الإشارة الأصلية

تعيد الدالة `recover_orignal_signal` الإشارة الأصلية من مصفوفة قناة بحجم `2 × 2` ومتجه استقبال بحجم عنصرين.

```rust
use nalgebra::{SMatrix, Vector2};
use complex::Complex;

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

let result = Complex::recover_orignal_signal(&channel, &received);
```

يرجع الخطأ `HardwareMathError::UninvertibleCannelMatrix` إذا كانت محددة مصفوفة القناة تساوي صفرًا.

## التحويل التجريبي للإشارة

تعمل الدالة `hardwarehackphy` على مقطع من ثمانية أعداد مركبة وتعيد مصفوفة ذات ثمانية مخرجات. يمكن استخدامها كبداية لتنفيذ تحويل شبيه بتحويلات الإشارة ضمن أنظمة الاتصالات.

```rust
let input = [
    Complex::new(1.0, 0.0),
    Complex::new(1.0, 0.0),
    Complex::new(1.0, 0.0),
    Complex::new(1.0, 0.0),
    Complex::new(1.0, 0.0),
    Complex::new(1.0, 0.0),
    Complex::new(1.0, 0.0),
    Complex::new(1.0, 0.0),
];

let output = Complex::hardwarehackphy(&input);
```

## التشغيل والاختبار

شغّل الاختبارات بالمشروع:

```bash
cargo test
```

راجع الحزمة:

```bash
cargo check
```

## ملاحظات حالية

- الدالة `exp_simd` متاحة فقط على معمارية `x86_64`.
- بعض دوال معالجة الهاردوير ودوال الاتصال لا تزال تجريبية ويُنصح بمراجعة الصيغ الرياضية واختبارات الدقة قبل استخدامها في نظام إنتاجي.
- اسم الخطأ `UninvertibleCannelMatrix` واسم الدالة `recover_orignal_signal` مستخدمان كما هو في واجهة المشروع الحالية.
