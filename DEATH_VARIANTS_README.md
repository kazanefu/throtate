# Death ブロックのバリエーション

このドキュメントでは、新しく追加された Death ブロックのバリエーションについて説明します。

## 概要

従来の `Death` ブロックに加えて、以下の2つの新しいバリエーションを追加しました：

1. **DeathCustom** - サイズと回転を指定可能な固定 Death ブロック
2. **DynamicDeath** - 動的な物理挙動を持つ Death ブロック

## 1. DeathCustom

### 説明
固定された（動かない）Death ブロックで、幅、高さ、回転角度をカスタマイズできます。

### パラメータ
- `width`: f32 - ブロックの幅（必須）
- `height`: f32 - ブロックの高さ（必須）
- `rotation`: Option<f32> - 回転角度（ラジアン、省略可能、デフォルト: 0.0）

### 使用例

```ron
// 45度回転した長方形の Death ブロック
(x: 300.0, y: 0.0, kind: DeathCustom(width: 200.0, height: 50.0, rotation: Some(0.785)))

// 回転なしの正方形 Death ブロック
(x: 400.0, y: 0.0, kind: DeathCustom(width: 100.0, height: 100.0, rotation: None))

// 90度回転した細長い Death ブロック
(x: 500.0, y: 0.0, kind: DeathCustom(width: 150.0, height: 30.0, rotation: Some(1.57)))
```

### 活用シーン
- 斜めの障害物を作成
- 細長い壁や床を作成
- 複雑な形状の障害物配置

## 2. DynamicDeath

### 説明
物理エンジンによって動く Death ブロックです。重力の影響を受け、他のオブジェクトと衝突します。

### パラメータ
- `width`: Option<f32> - ブロックの幅（省略可能、デフォルト: box_size）
- `height`: Option<f32> - ブロックの高さ（省略可能、デフォルト: box_size）
- `rotation`: Option<f32> - 初期回転角度（ラジアン、省略可能、デフォルト: 0.0）
- `gravity_scale`: Option<f32> - 重力スケール（省略可能、デフォルト: 1.0）
- `linear_damping`: Option<f32> - 線形減衰（省略可能、デフォルト: 0.0）
- `angular_damping`: Option<f32> - 角度減衰（省略可能、デフォルト: 0.0）
- `density_scale`: Option<f32> - 密度スケール（省略可能、デフォルト: 1.0）
- `restitution_coefficient`: Option<f32> - 反発係数（省略可能、デフォルト: 0.0）

### 使用例

```ron
// 通常の重力で落ちる Death ブロック（よく跳ねる）
(x: 600.0, y: 200.0, kind: DynamicDeath(
    width: Some(80.0),
    height: Some(80.0),
    rotation: Some(0.0),
    gravity_scale: Some(1.0),
    linear_damping: None,
    angular_damping: None,
    density_scale: Some(2.0),
    restitution_coefficient: Some(0.5)
))

// 低重力でゆっくり動く Death ブロック（高い反発）
(x: 700.0, y: 250.0, kind: DynamicDeath(
    width: Some(60.0),
    height: Some(120.0),
    rotation: Some(0.3),
    gravity_scale: Some(0.5),
    linear_damping: Some(0.5),
    angular_damping: Some(0.5),
    density_scale: Some(1.0),
    restitution_coefficient: Some(0.8)
))

// デフォルト値を使用した簡易版（高重力）
(x: 800.0, y: 200.0, kind: DynamicDeath(
    width: None,
    height: None,
    rotation: None,
    gravity_scale: Some(2.0),
    linear_damping: None,
    angular_damping: None,
    density_scale: None,
    restitution_coefficient: None
))
```

### 活用シーン
- 落下してくる障害物
- 振り子のように揺れる障害物
- プレイヤーが押せる危険なブロック
- 弾むDeath ブロック

## パラメータの説明

### gravity_scale（重力スケール）
- `1.0`: 通常の重力
- `0.0`: 無重力（浮遊）
- `2.0`: 2倍の重力（速く落ちる）
- 負の値: 上向きの重力

### linear_damping（線形減衰）
- `0.0`: 減衰なし（摩擦なし）
- `0.5`: 中程度の減衰
- `1.0`: 高い減衰（すぐに止まる）

### angular_damping（角度減衰）
- `0.0`: 回転の減衰なし
- `0.5`: 中程度の減衰
- `1.0`: 高い減衰（回転がすぐに止まる）

### density_scale（密度スケール）
- 質量に影響します
- `1.0`: 標準
- `2.0`: 2倍重い
- `0.5`: 半分の重さ

### restitution_coefficient（反発係数）
- `0.0`: 全く跳ねない
- `0.5`: 中程度に跳ねる
- `1.0`: 完全に跳ね返る（エネルギー保存）

## テストコース

`assets/courses_ron/test.ron` に使用例が追加されています。
実際にゲーム内で動作を確認できます。

## コードレビュー対応

### cargo clippy 対応
- `death_box_custom_bundle`: 引数が多すぎる警告を回避するため、`DeathCustomParams` 構造体を導入
- `death_box_dynamic_bundle`: `EntityKind` を直接渡すことで、パターンマッチングを関数内で行う

### メッシュと衝突判定の一致
- カスタムサイズの `Rectangle` メッシュを動的に生成
- Collider のサイズも同じ width/height を使用
- 見た目と物理挙動が完全に一致するように修正

## 実装詳細

### 変更ファイル
1. `src/course/mod.rs` - EntityKind に DeathCustom と DynamicDeath を追加
2. `src/course/course_items/death_box.rs` - death_box_custom_bundle と death_box_dynamic_bundle を追加
   - カスタムサイズのメッシュを動的に生成して、見た目と衝突判定が一致するように実装
   - DeathCustomParams 構造体を導入して、cargo clippy の警告を回避
3. `src/course/spawn.rs` - 新しいエンティティタイプのスポーン処理を追加
   - Mesh Assets へのアクセスを追加
   - DynamicDeath は EntityKind を直接渡すことで、clippy 警告を回避
4. `assets/courses_ron/test.ron` - 使用例を追加

### コンポーネント
- 両方とも `Death` コンポーネントを持ちます
- DeathCustom: `RigidBody::Fixed`（固定）
- DynamicDeath: `RigidBody::Dynamic`（動的）

### メッシュとマテリアル
- 各 Death ブロックは指定されたサイズに合わせて動的に `Rectangle` メッシュを生成します
- DeathMaterial（カスタムシェーダー）を使用して、統一された見た目を維持します
- 見た目（Mesh）と衝突判定（Collider）が完全に一致します
