#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_snake_case)]

use std::f32;

/*
 * Use external library "macroclassrender"
 */
#[macro_use]
extern crate macroclassrender;

mod my {
/*
 * set derives macro used
 */
#[derive(Functions, Constructor, Getters, Setters, Debug)]
pub struct Vector2 {

    /*
     * {Methods} {Visibility mutable}
     * {getter and setter}
     */
    #[get = "pub"] #[set = "pub"]
    x : f32,
    /*
     * {Methods} {Visibility mutable}
     * {getter and setter}
     */
    #[get = "pub"] #[set = "pub"]
    y : f32,

    /*
     * {Contructor} {Visibility mutable}
     * create method "new(...)" and "__constructor(...)"
     * return {Vector2}
     */
    #[__constructor = "(x : f32, y : f32) -> Vector2(x : f32, y : f32, import : String) {
        this.x = x;
        this.y = y;
    }"]

    /*
     * {Method} {Visibility mutable}
     * return {&Vector2}
     */
    #[__function = "public mutable &Vector2 set(x : f32, y : f32) {
		this.x = x;
		this.y = y;
		return this;
	}"]

    /*
     * {Method} {Visibility immutable}
     * return {f32}
     */
    #[__function = "public immutable f32 sqrt() {
		return this.x * this.x + this.y * this.y;
	}"]

    /*
     * {Method} {Visibility immutable}
     * return {f32}
     */
    #[__function = "public immutable f32 magnitude() {
        let val = this.sqrt();
        return val.sqrt();
	}"]

    /*
     * {Method} {Visibility immutable}
     * return {Vector2}
     */
    #[__function = "public immutable Vector2 normalize() {
		return Vector2::new(this.x / this.magnitude(), this.y / this.magnitude());
	}"]

    /*
     * {Method} {Visibility immutable}
     * return {Vector2}
     */
    #[__function = "public immutable Vector2 cross(r : &Vector2) {
		return Vector2::new(r.x, -r.y);
	}"]

    /*
     * {Method} {Visibility immutable}
     * return {f32}
     */
    #[__function = "public immutable f32 dot(r : &Vector2) {
		return this.x * r.x + this.y * r.y;
	}"]

    /*
     * {Method} {Visibility immutable}
     * return {f32}
     */
    #[__function = "public immutable f32 max() {
		return this.x.max(this.y);
	}"]

    /*
     * {Method} {Visibility immutable}
     * return {f32}
     */
    #[__function = "public immutable f32 min() {
		return this.x.min(this.y);
	}"]

    /*
     * {Method} {Visibility mutable}
     * return {&Vector2}
     */
    #[__function = "public mutable &Vector2 add(val : f32) {
		this.x += val;
		this.y += val;
		return this;
	}"]

    /*
     * {Method} {Visibility mutable}
     * return {&Vector2}
     */
    #[__function = "public mutable &Vector2 sub(val : f32) {
		this.x -= val;
		this.y -= val;
		return this;
	}"]

    /*
     * {Method} {Visibility mutable}
     * return {&Vector2}
     */
    #[__function = "public mutable &Vector2 mul(val : f32) {
		this.x *= val;
		this.y *= val;
		return this;
	}"]

    /*
     * {Method} {Visibility mutable}
     * return {&Vector2}
     */
    #[__function = "public mutable &Vector2 div(val : f32) {
		this.x /= val;
		this.y /= val;
		return this;
	}"]

    /*
     * {Method} {Visibility mutable}
     * return {&Vector2}
     */
    #[__function = "public mutable &Vector2 add_vector(vec : &Vector2) {
		this.x += vec.x;
		this.y += vec.y;
		return this;
	}"]

    /*
     * {Method} {Visibility mutable}
     * return {&Vector2}
     */
    #[__function = "public mutable &Vector2 sub_vector(vec : &Vector2) {
		this.x -= vec.x;
		this.y -= vec.y;
		return this;
	}"]

    /*
     * {Method} {Visibility mutable}
     * return {&Vector2}
     */
    #[__function = "public mutable &Vector2 mul_vector(vec : &Vector2) {
		this.x *= vec.x;
		this.y *= vec.y;
		return this;
	}"]

    #[__function = "public mutable &Vector2 div_vector(vec : &Vector2) {
		this.x /= vec.x;
		this.y /= vec.y;
		return this;
	}"]

    /*
     * {Method} {Visibility immutable}
     * return {Vector2}
     */
    #[__function = "public immutable Vector2 clone() {
		return Vector2::new(this.x, this.y);
	}"]

    /*
     * {Method} {Visibility immutable}
     * return {bool}
     */
    #[__function = "public immutable bool equals(v : &Vector2) {
		return this.x == v.x && this.y == v.y;
	}"]

    /*
     * {Method} {Visibility public and immutable}
     * return {String}
     */
    #[__function = "public immutable String toString() {
        return format!(\"x: {}, y: {}\", this.x, this.y);
    }"]

    /*
     * {Method} {Visibility public and immutable}
     * return {void}
     */
    #[__function = "public immutable void debug() {
		println!(\"Debug Vector2 [{}]\", this.toString());
	}"]

    /*
     * {Method} {Visibility private and immutable}
     * return {void}
     */
    #[__function = "public immutable void testvisibility() {
		println!(\"Debug Vector2 [{}]\", this.toString());
	}"]
    pub import : String
}

}

fn main() {
    let v = my::Vector2::new(50f32,10f32);
    let tmp = v.clone();

    v.debug();
    tmp.debug();
    tmp.testvisibility();
    println!("v: {}", v.toString());

    println!("v: {}", v.import);
}
