use crate::{
    attr::{Attr, AttrClass, AttrContext, AttrField, SizedField},
    fixed::{Fixed, MutFixed},
    slice::ByteSlice,
    token::Token,
    variant::Variant,
};
use std::{
    fmt,
    marker::PhantomData,
    ops::{Deref, DerefMut, Range},
    slice,
};

#[repr(C)]
pub struct LayerStackData {
    pub children: Vec<MutFixed<Layer>>,
}

/// A mutable proxy for a layer object.
#[repr(C)]
pub struct LayerStack<'a> {
    data: *mut LayerStackData,
    depth: u8,
    add_child: extern "C" fn(*mut LayerStackData, *mut Layer),
    children_len: extern "C" fn(*const LayerStackData) -> u64,
    children_data: extern "C" fn(*const LayerStackData) -> *const MutFixed<Layer>,
    layer: *mut Layer,
    phantom: PhantomData<&'a ()>,
}

impl<'a> LayerStack<'a> {
    pub fn from_mut_ref(stack: &'a mut LayerStackData, layer: &'a mut Layer) -> LayerStack<'a> {
        LayerStack {
            data: stack,
            depth: 0,
            add_child: abi_add_child,
            children_len: abi_children_len,
            children_data: abi_children_data,
            layer,
            phantom: PhantomData,
        }
    }

    /// Returns the ID of self.
    pub fn id(&self) -> Token {
        self.deref().id()
    }

    /// Returns the type of self.
    pub fn data(&self) -> ByteSlice {
        self.deref().data()
    }

    /// Returns the slice of attributes.
    pub fn attrs(&self) -> impl Iterator<Item = Attr> {
        self.deref().attrs()
    }

    /// Find the attribute in the Layer.
    pub fn attr<T: Into<Token>>(&self, id: T) -> Option<Attr> {
        self.deref().attr(id)
    }

    /// Adds an attribute to the Layer.
    pub fn add_attr<C: Into<Fixed<AttrClass>>>(&mut self, attr: C, range: Range<usize>) {
        self.deref_mut().add_attr(attr, range);
    }

    /// Returns the slice of payloads.
    pub fn payloads(&self) -> impl Iterator<Item = &Payload> {
        self.deref().payloads()
    }

    /// Adds a payload to the Layer.
    pub fn add_payload(&mut self, payload: Payload) {
        self.deref_mut().add_payload(payload);
    }

    pub fn add_child<T: Into<MutFixed<Layer>>>(&mut self, layer: T) {
        (self.add_child)(self.data, layer.into().as_mut_ptr());
    }

    pub fn top(&self) -> Option<&Layer> {
        self.children().iter().rev().next().map(Deref::deref)
    }

    pub fn bottom(&self) -> Option<&Layer> {
        self.children().iter().next().map(Deref::deref)
    }

    fn children(&self) -> &[MutFixed<Layer>] {
        let data = (self.children_data)(self.data);
        let len = (self.children_len)(self.data) as usize;
        unsafe { slice::from_raw_parts(data, len) }
    }
}

impl<'a> Deref for LayerStack<'a> {
    type Target = Layer;

    fn deref(&self) -> &Layer {
        unsafe { &*self.layer }
    }
}

impl<'a> DerefMut for LayerStack<'a> {
    fn deref_mut(&mut self) -> &mut Layer {
        unsafe { &mut *self.layer }
    }
}

extern "C" fn abi_add_child(data: *mut LayerStackData, child: *mut Layer) {
    unsafe { (*data).children.push(MutFixed::from_ptr(child)) }
}

extern "C" fn abi_children_len(data: *const LayerStackData) -> u64 {
    unsafe { (*data).children.len() as u64 }
}

extern "C" fn abi_children_data(data: *const LayerStackData) -> *const MutFixed<Layer> {
    unsafe { (*data).children.as_ptr() }
}

#[repr(C)]
struct BoundAttr {
    attr: Fixed<AttrClass>,
    range: Range<usize>,
}

/// A layer object.
#[repr(C)]
pub struct Layer {
    class: Fixed<LayerClass>,
    data: ByteSlice,
    attrs: Vec<BoundAttr>,
    payloads: Vec<Payload>,
}

unsafe impl Send for Layer {}

impl Layer {
    /// Creates a new Layer.
    pub fn new<C: Into<Fixed<LayerClass>>, B: Into<ByteSlice>>(class: C, data: B) -> Layer {
        Layer {
            class: class.into(),
            data: data.into(),
            attrs: Vec::new(),
            payloads: Vec::new(),
        }
    }

    /// Returns the ID of self.
    pub fn id(&self) -> Token {
        self.class.id()
    }

    /// Returns the type of self.
    pub fn data(&self) -> ByteSlice {
        self.class.data(self)
    }

    /// Returns the slice of headers.
    pub fn header(&self) -> &Fixed<AttrClass> {
        self.class.header()
    }

    /// Returns the slice of attributes.
    pub fn attrs(&self) -> impl Iterator<Item = Attr> {
        self.class.attrs(self)
    }

    /// Find the attribute in the Layer.
    pub fn attr<T: Into<Token>>(&self, id: T) -> Option<Attr> {
        let id = id.into();

        AttrClass::expand(self.class.header(), &self.data, None)
            .into_iter()
            .chain(
                self.attrs
                    .iter()
                    .map(|c| {
                        AttrClass::expand(&c.attr, &self.data(), Some(c.range.clone())).into_iter()
                    })
                    .flatten(),
            )
            .find(|attr| attr.is_match(id))
    }

    /// Adds an attribute to the Layer.
    pub fn add_attr<C: Into<Fixed<AttrClass>>>(&mut self, attr: C, range: Range<usize>) {
        let func = self.class.add_attr;
        (func)(
            self,
            BoundAttr {
                attr: attr.into(),
                range: (range.start * 8)..(range.end * 8),
            },
        );
    }

    /// Returns the slice of payloads.
    pub fn payloads(&self) -> impl Iterator<Item = &Payload> {
        self.class.payloads(self)
    }

    /// Adds a payload to the Layer.
    pub fn add_payload(&mut self, payload: Payload) {
        let func = self.class.add_payload;
        (func)(self, payload);
    }
}

impl fmt::Debug for Layer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Layer {:?}", self.id())
    }
}

impl Into<MutFixed<Layer>> for Layer {
    fn into(self) -> MutFixed<Layer> {
        MutFixed::new(self)
    }
}

/// A payload object.
#[repr(C)]
pub struct Payload {
    data: *const u8,
    len: u64,
    id: Token,
    typ: Token,
}

impl Payload {
    /// Creates a new payload.
    pub fn new<B: Into<ByteSlice>, T: Into<Token>>(data: B, id: T) -> Payload {
        Self::with_typ(data, id, "")
    }

    /// Creates a new payload with the given type.
    pub fn with_typ<B: Into<ByteSlice>, T: Into<Token>, U: Into<Token>>(
        data: B,
        id: T,
        typ: U,
    ) -> Payload {
        let data: ByteSlice = data.into();
        Self {
            data: data.as_ptr(),
            len: data.len() as u64,
            id: id.into(),
            typ: typ.into(),
        }
    }

    /// Returns the ID of self.
    pub fn id(&self) -> Token {
        self.id
    }

    /// Returns the type of self.
    pub fn typ(&self) -> Token {
        self.typ
    }

    /// Returns the data of self.
    pub fn data(&self) -> ByteSlice {
        unsafe { ByteSlice::from_raw_parts(self.data, self.len as usize) }
    }
}

/// A builder object for LayerClass.
pub struct LayerClassBuilder {
    header: Fixed<AttrClass>,
}

impl LayerClassBuilder {
    /// Builds a new LayerClass.
    pub fn build(self) -> LayerClass {
        LayerClass {
            get_id: abi_id,
            data: abi_data,
            attrs_len: abi_attrs_len,
            attrs_data: abi_attrs_data,
            add_attr: abi_add_attr,
            payloads_len: abi_payloads_len,
            payloads_data: abi_payloads_data,
            add_payload: abi_add_payload,
            header: self.header,
        }
    }
}

pub struct LayerType<T> {
    field: T,
    layer: Fixed<LayerClass>,
}

impl<T: fmt::Debug> fmt::Debug for LayerType<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("LayerType")
            .field("field", &self.field)
            .field("layer", &self.layer)
            .finish()
    }
}

impl<T> Deref for LayerType<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.field
    }
}

impl<T> AsRef<Fixed<LayerClass>> for LayerType<T> {
    fn as_ref(&self) -> &Fixed<LayerClass> {
        &self.layer
    }
}

impl<I: Into<Variant>, T: AttrField<I = I> + SizedField> LayerType<T> {
    pub fn new<D: Into<Token>>(id: D, field: T) -> Self {
        let ctx = AttrContext {
            path: id.into().to_string(),
            typ: "@layer".into(),
            ..Default::default()
        };
        let class = field.class(&ctx, field.bit_size(), None).build();
        let layer = Fixed::new(LayerClass {
            get_id: abi_id,
            data: abi_data,
            attrs_len: abi_attrs_len,
            attrs_data: abi_attrs_data,
            add_attr: abi_add_attr,
            payloads_len: abi_payloads_len,
            payloads_data: abi_payloads_data,
            add_payload: abi_add_payload,
            header: Fixed::new(class),
        });
        Self { field, layer }
    }
}

/// A layer class object.
#[repr(C)]
pub struct LayerClass {
    get_id: extern "C" fn(*const LayerClass) -> Token,
    data: extern "C" fn(*const Layer, *mut u64) -> *const u8,
    attrs_len: extern "C" fn(*const Layer) -> u64,
    attrs_data: extern "C" fn(*const Layer) -> *const BoundAttr,
    add_attr: extern "C" fn(*mut Layer, BoundAttr),
    payloads_len: extern "C" fn(*const Layer) -> u64,
    payloads_data: extern "C" fn(*const Layer) -> *const Payload,
    add_payload: extern "C" fn(*mut Layer, Payload),
    header: Fixed<AttrClass>,
}

impl LayerClass {
    /// Creates a new builder object for LayerClass.
    pub fn builder<H: Into<Fixed<AttrClass>>>(header: H) -> LayerClassBuilder {
        LayerClassBuilder {
            header: header.into(),
        }
    }

    pub fn new<I: Into<Variant>, T: Into<Token>, A: SizedField + AttrField<I = I>>(
        id: T,
        attr: &A,
    ) -> LayerClass {
        let ctx = AttrContext {
            path: id.into().to_string(),
            typ: "@layer".into(),
            ..Default::default()
        };
        let class = attr.class(&ctx, attr.bit_size(), None).build();
        LayerClass {
            get_id: abi_id,
            data: abi_data,
            attrs_len: abi_attrs_len,
            attrs_data: abi_attrs_data,
            add_attr: abi_add_attr,
            payloads_len: abi_payloads_len,
            payloads_data: abi_payloads_data,
            add_payload: abi_add_payload,
            header: Fixed::new(class),
        }
    }

    fn id(&self) -> Token {
        (self.get_id)(self)
    }

    fn header(&self) -> &Fixed<AttrClass> {
        &self.header
    }

    fn data(&self, layer: &Layer) -> ByteSlice {
        let mut len = 0;
        let data = (self.data)(layer, &mut len);
        unsafe { ByteSlice::from_raw_parts(data, len as usize) }
    }

    fn attrs(&self, layer: &Layer) -> impl Iterator<Item = Attr> {
        let data = (self.attrs_data)(layer);
        let len = (self.attrs_len)(layer) as usize;
        let attrs = unsafe { slice::from_raw_parts(data, len) }
            .iter()
            .map(|c| AttrClass::expand(&c.attr, &layer.data(), Some(c.range.clone())).into_iter())
            .flatten()
            .collect::<Vec<_>>();
        attrs.into_iter()
    }

    fn payloads(&self, layer: &Layer) -> impl Iterator<Item = &Payload> {
        let data = (self.payloads_data)(layer);
        let len = (self.payloads_len)(layer) as usize;
        unsafe { slice::from_raw_parts(data, len) }.iter()
    }
}

impl fmt::Debug for LayerClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("LayerClass")
            .field("header", &self.header)
            .finish()
    }
}

impl Into<Fixed<LayerClass>> for &'static LayerClass {
    fn into(self) -> Fixed<LayerClass> {
        Fixed::from_static(self)
    }
}

extern "C" fn abi_id(class: *const LayerClass) -> Token {
    unsafe { (*class).header.id() }
}

extern "C" fn abi_data(layer: *const Layer, len: *mut u64) -> *const u8 {
    unsafe {
        let data = &(*layer).data;
        *len = data.len() as u64;
        data.as_ptr()
    }
}

extern "C" fn abi_attrs_len(layer: *const Layer) -> u64 {
    unsafe { (*layer).attrs.len() as u64 }
}

extern "C" fn abi_attrs_data(layer: *const Layer) -> *const BoundAttr {
    unsafe { (*layer).attrs.as_ptr() }
}

extern "C" fn abi_add_attr(layer: *mut Layer, attr: BoundAttr) {
    let attrs = unsafe { &mut (*layer).attrs };
    attrs.push(attr);
}

extern "C" fn abi_payloads_len(layer: *const Layer) -> u64 {
    unsafe { (*layer).payloads.len() as u64 }
}

extern "C" fn abi_payloads_data(layer: *const Layer) -> *const Payload {
    unsafe { (*layer).payloads.as_ptr() }
}

extern "C" fn abi_add_payload(layer: *mut Layer, payload: Payload) {
    let payloads = unsafe { &mut (*layer).payloads };
    payloads.push(payload);
}

#[cfg(test)]
mod tests {
    use crate::{
        attr::{Attr, AttrClass},
        cast::Cast,
        fixed::Fixed,
        layer::{Layer, LayerClass, Payload},
        slice::ByteSlice,
        token::Token,
        variant::Variant,
    };
    use std::io::Result;

    #[test]
    fn id() {
        let id = Token::from(123);
        let attr = Fixed::new(AttrClass::builder(id).build());
        let class = Fixed::new(LayerClass::builder(attr).build());
        let layer = Layer::new(class, ByteSlice::new());
        assert_eq!(layer.id(), id);
    }

    #[test]
    fn data() {
        let data = b"hello";
        let attr = Fixed::new(AttrClass::builder(Token::null()).build());
        let class = Fixed::new(LayerClass::builder(attr).build());
        let layer = Layer::new(class, ByteSlice::from(&data[..]));
        assert_eq!(layer.data(), ByteSlice::from(&data[..]));
    }

    #[test]
    fn payloads() {
        let attr = Fixed::new(AttrClass::builder(Token::null()).build());
        let class = Fixed::new(LayerClass::builder(attr).build());
        let mut layer = Layer::new(class, ByteSlice::new());
        assert!(layer.payloads().next().is_none());

        let count = 100;
        let data = b"hello";

        for i in 0..count {
            layer.add_payload(Payload::new(ByteSlice::from(&data[..]), Token::from(i)));
        }

        let mut iter = layer.payloads();
        for i in 0..count {
            let payload = iter.next().unwrap();
            assert_eq!(payload.data(), ByteSlice::from(&data[..]));
            assert_eq!(payload.id(), Token::from(i));
        }
        assert!(iter.next().is_none());
    }

    #[test]
    fn attrs() {
        let attr = Fixed::new(AttrClass::builder(Token::null()).build());
        let class = Fixed::new(LayerClass::builder(attr).build());
        let mut layer = Layer::new(class, ByteSlice::new());
        assert!(layer.attrs().next().is_none());

        #[derive(Clone)]
        struct TestCast {}

        impl Cast for TestCast {
            fn cast(&self, _attr: &Attr, _: &ByteSlice) -> Result<Variant> {
                Ok(Variant::Nil)
            }
        }
        let class = Fixed::new(
            AttrClass::builder("nil")
                .typ("@nil")
                .cast(&TestCast {})
                .build(),
        );

        let count = 100;
        for i in 0..count {
            layer.add_attr(class.clone(), 0..i);
        }
        let mut iter = layer.attrs();
        for i in 0..count {
            let attr = iter.next().unwrap();
            assert_eq!(attr.id(), Token::from("nil"));
            assert_eq!(attr.typ(), Token::from("@nil"));
            assert_eq!(attr.range(), 0..i);
        }
        assert!(iter.next().is_none());
    }
}
