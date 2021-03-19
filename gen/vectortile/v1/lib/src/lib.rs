#![doc = "# Resources and Methods\n    * [featuretiles](resources/featuretiles/struct.FeaturetilesActions.html)\n      * [*get*](resources/featuretiles/struct.GetRequestBuilder.html)\n    * [terraintiles](resources/terraintiles/struct.TerraintilesActions.html)\n      * [*get*](resources/terraintiles/struct.GetRequestBuilder.html)\n"]
pub mod scopes {}
pub mod schemas {
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Area {
        #[doc = "The z-order of this geometry when rendered on a flat basemap. Geometry with a lower z-order should be rendered beneath geometry with a higher z-order. This z-ordering does not imply anything about the altitude of the area relative to the ground, but it can be used to prevent z-fighting. Unlike Area.z_order this can be used to compare with Line.basemap_z_order, and in fact may yield more accurate rendering (where a line may be rendered beneath an area)."]
        #[serde(
            rename = "basemapZOrder",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub basemap_z_order: ::std::option::Option<crate::schemas::BasemapZOrder>,
        #[doc = "True if the polygon is not entirely internal to the feature that it belongs to: that is, some of the edges are bordering another feature."]
        #[serde(
            rename = "hasExternalEdges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub has_external_edges: ::std::option::Option<bool>,
        #[doc = "When has_external_edges is true, the polygon has some edges that border another feature. This field indicates the internal edges that do not border another feature. Each value is an index into the vertices array, and denotes the start vertex of the internal edge (the next vertex in the boundary loop is the end of the edge). If the selected vertex is the last vertex in the boundary loop, then the edge between that vertex and the starting vertex of the loop is internal. This field may be used for styling. For example, building parapets could be placed only on the external edges of a building polygon, or water could be lighter colored near the external edges of a body of water. If has_external_edges is false, all edges are internal and this field will be empty."]
        #[serde(
            rename = "internalEdges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub internal_edges: ::std::option::Option<Vec<i32>>,
        #[doc = "Identifies the boundary loops of the polygon. Only set for INDEXED_TRIANGLE polygons. Each value is an index into the vertices array indicating the beginning of a loop. For instance, values of [2, 5] would indicate loop_data contained 3 loops with indices 0-1, 2-4, and 5-end. This may be used in conjunction with the internal_edges field for styling polygon boundaries. Note that an edge may be on a polygon boundary but still internal to the feature. For example, a feature split across multiple tiles will have an internal polygon boundary edge along the edge of the tile."]
        #[serde(
            rename = "loopBreaks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub loop_breaks: ::std::option::Option<Vec<i32>>,
        #[doc = "The polygon encoding type used for this area."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::AreaType>,
        #[doc = "When the polygon encoding is of type INDEXED_TRIANGLES, this contains the indices of the triangle vertices in the vertex_offsets field. There are 3 vertex indices per triangle."]
        #[serde(
            rename = "triangleIndices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub triangle_indices: ::std::option::Option<Vec<i32>>,
        #[doc = "The vertices present in the polygon defining the area."]
        #[serde(
            rename = "vertexOffsets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub vertex_offsets: ::std::option::Option<crate::schemas::Vertex2DList>,
        #[doc = "The z-ordering of this area. Areas with a lower z-order should be rendered beneath areas with a higher z-order. This z-ordering does not imply anything about the altitude of the line relative to the ground, but it can be used to prevent z-fighting during rendering on the client. This z-ordering can only be used to compare areas, and cannot be compared with the z_order field in the Line message. The z-order may be negative or zero. Prefer Area.basemap_z_order."]
        #[serde(
            rename = "zOrder",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub z_order: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for Area {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Area {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AreaType {
        #[doc = "The polygon is a set of triangles with three vertex indices per triangle. The vertex indices can be found in the triangle_indices field. Indexed triangle polygons also contain information about boundary loops. These identify the loops at the boundary of the polygon and may be used in conjunction with the internal_edges field for styling. Boundary loops may represent either a hole or a disconnected component of the polygon. The following diagram shows an indexed triangle polygon with two boundary loops. (0) (4) / \\ / \\ / \\ / \\ (1)----(2) (3)----(5)"]
        IndexedTriangles,
        #[doc = "The first vertex in vertex_offset is the center of a triangle fan. The other vertices are arranged around this vertex in a fan shape. The following diagram showes a triangle fan polygon with the vertices labelled with their indices in the vertex_offset list. Triangle fan polygons always have a single boundary loop. Vertices may be in either a clockwise or counterclockwise order. (1) / \\ / \\ / \\ (0)-----(2) / \\ / / \\ / / \\ / (4)-----(3)"]
        TriangleFan,
        #[doc = "A strip of triangles, where each triangle uses the last edge of the previous triangle. Vertices may be in either a clockwise or counterclockwise order. Only polygons without the has_external_edges flag set will use triangle strips. (0) / \\ / \\ / \\ (2)-----(1) / \\ / / \\ / / \\ / (4)-----(3)"]
        TriangleStrip,
    }
    impl AreaType {
        pub fn as_str(self) -> &'static str {
            match self {
                AreaType::IndexedTriangles => "INDEXED_TRIANGLES",
                AreaType::TriangleFan => "TRIANGLE_FAN",
                AreaType::TriangleStrip => "TRIANGLE_STRIP",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AreaType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AreaType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AreaType, ()> {
            Ok(match s {
                "INDEXED_TRIANGLES" => AreaType::IndexedTriangles,
                "TRIANGLE_FAN" => AreaType::TriangleFan,
                "TRIANGLE_STRIP" => AreaType::TriangleStrip,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AreaType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AreaType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AreaType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "INDEXED_TRIANGLES" => AreaType::IndexedTriangles,
                "TRIANGLE_FAN" => AreaType::TriangleFan,
                "TRIANGLE_STRIP" => AreaType::TriangleStrip,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AreaType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AreaType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct BasemapZOrder {
        #[doc = "The second most significant component of the ordering of a component to be rendered onto the basemap."]
        #[serde(
            rename = "zGrade",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub z_grade: ::std::option::Option<i32>,
        #[doc = "The most significant component of the ordering of a component to be rendered onto the basemap."]
        #[serde(
            rename = "zPlane",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub z_plane: ::std::option::Option<i32>,
        #[doc = "The least significant component of the ordering of a component to be rendered onto the basemap."]
        #[serde(
            rename = "zWithinGrade",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub z_within_grade: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for BasemapZOrder {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BasemapZOrder {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ExtrudedArea {
        #[doc = "The area representing the footprint of the extruded area."]
        #[serde(
            rename = "area",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub area: ::std::option::Option<crate::schemas::Area>,
        #[doc = "The z-value in local tile coordinates where the extruded area ends."]
        #[serde(
            rename = "maxZ",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_z: ::std::option::Option<i32>,
        #[doc = "The z-value in local tile coordinates where the extruded area begins. This is non-zero for extruded areas that begin off the ground. For example, a building with a skybridge may have an extruded area component with a non-zero min_z."]
        #[serde(
            rename = "minZ",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub min_z: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for ExtrudedArea {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ExtrudedArea {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Feature {
        #[doc = "The localized name of this feature. Currently only returned for roads."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "The geometry of this feature, representing the space that it occupies in the world."]
        #[serde(
            rename = "geometry",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub geometry: ::std::option::Option<crate::schemas::Geometry>,
        #[doc = "Place ID of this feature, suitable for use in Places API details requests."]
        #[serde(
            rename = "placeId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub place_id: ::std::option::Option<String>,
        #[doc = "The type of this feature."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::FeatureType>,
        #[doc = "Relations to other features."]
        #[serde(
            rename = "relations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub relations: ::std::option::Option<Vec<crate::schemas::Relation>>,
        #[doc = "Metadata for features with the SEGMENT FeatureType."]
        #[serde(
            rename = "segmentInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment_info: ::std::option::Option<crate::schemas::SegmentInfo>,
    }
    impl ::google_field_selector::FieldSelector for Feature {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Feature {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum FeatureType {
        #[doc = "Top-level divisions within a country, such as prefectures or states."]
        AdministrativeArea1,
        #[doc = "Major through road that's expected to carry large volumes of traffic."]
        ArterialRoad,
        #[doc = "A financial institution that offers services to the general public."]
        Bank,
        #[doc = "A business serving alcoholic drinks to be consumed onsite."]
        Bar,
        #[doc = "A pebbly or sandy shore along the edge of a sea or lake."]
        Beach,
        #[doc = "A business that sells coffee, tea, and sometimes small meals."]
        Cafe,
        #[doc = "A highway with grade-separated crossings that is accessed exclusively by ramps. These are usually called \"freeways\" or \"motorways\". The enable_detailed_highway_types request flag must be set in order for this type to be returned."]
        ControlledAccessHighway,
        #[doc = "A venue for private and public events."]
        EventVenue,
        #[doc = "Unknown feature type."]
        FeatureTypeUnspecified,
        #[doc = "Services which are part of the road network but are not roads."]
        Ferry,
        #[doc = "A path that's primarily intended for use by pedestrians and/or cyclists."]
        Footpath,
        #[doc = "Area of land covered by trees."]
        Forest,
        #[doc = "A major road including freeways and state highways."]
        Highway,
        #[doc = "A small city street, typically for travel in a residential neighborhood."]
        LocalRoad,
        #[doc = "Cities, towns, and other municipalities."]
        Locality,
        #[doc = "A place that provides any type of lodging for travelers."]
        Lodging,
        #[doc = "Outdoor areas such as parks and botanical gardens."]
        Park,
        #[doc = "Political entities, such as provinces and districts."]
        Political,
        #[doc = "Tracks intended for use by trains."]
        Rail,
        #[doc = "Non-water areas such as parks and forest."]
        Region,
        #[doc = "A business that prepares meals on-site for service to customers."]
        Restaurant,
        #[doc = "A way leading from one place to another intended for use by vehicles."]
        Road,
        #[doc = "Institution where young people receive general (not vocation or professional) education."]
        School,
        #[doc = "Segments such as roads and train lines."]
        Segment,
        #[doc = "A structure containing a business or businesses that sell goods."]
        Shopping,
        #[doc = "Structures such as buildings and bridges."]
        Structure,
        #[doc = "Divisions within a locality like a borough or ward."]
        Sublocality,
        #[doc = "Place of interest to tourists, typically for natural or cultural value."]
        TouristDestination,
        #[doc = "Water features such as rivers and lakes."]
        Water,
    }
    impl FeatureType {
        pub fn as_str(self) -> &'static str {
            match self {
                FeatureType::AdministrativeArea1 => "ADMINISTRATIVE_AREA1",
                FeatureType::ArterialRoad => "ARTERIAL_ROAD",
                FeatureType::Bank => "BANK",
                FeatureType::Bar => "BAR",
                FeatureType::Beach => "BEACH",
                FeatureType::Cafe => "CAFE",
                FeatureType::ControlledAccessHighway => "CONTROLLED_ACCESS_HIGHWAY",
                FeatureType::EventVenue => "EVENT_VENUE",
                FeatureType::FeatureTypeUnspecified => "FEATURE_TYPE_UNSPECIFIED",
                FeatureType::Ferry => "FERRY",
                FeatureType::Footpath => "FOOTPATH",
                FeatureType::Forest => "FOREST",
                FeatureType::Highway => "HIGHWAY",
                FeatureType::LocalRoad => "LOCAL_ROAD",
                FeatureType::Locality => "LOCALITY",
                FeatureType::Lodging => "LODGING",
                FeatureType::Park => "PARK",
                FeatureType::Political => "POLITICAL",
                FeatureType::Rail => "RAIL",
                FeatureType::Region => "REGION",
                FeatureType::Restaurant => "RESTAURANT",
                FeatureType::Road => "ROAD",
                FeatureType::School => "SCHOOL",
                FeatureType::Segment => "SEGMENT",
                FeatureType::Shopping => "SHOPPING",
                FeatureType::Structure => "STRUCTURE",
                FeatureType::Sublocality => "SUBLOCALITY",
                FeatureType::TouristDestination => "TOURIST_DESTINATION",
                FeatureType::Water => "WATER",
            }
        }
    }
    impl ::std::convert::AsRef<str> for FeatureType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for FeatureType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<FeatureType, ()> {
            Ok(match s {
                "ADMINISTRATIVE_AREA1" => FeatureType::AdministrativeArea1,
                "ARTERIAL_ROAD" => FeatureType::ArterialRoad,
                "BANK" => FeatureType::Bank,
                "BAR" => FeatureType::Bar,
                "BEACH" => FeatureType::Beach,
                "CAFE" => FeatureType::Cafe,
                "CONTROLLED_ACCESS_HIGHWAY" => FeatureType::ControlledAccessHighway,
                "EVENT_VENUE" => FeatureType::EventVenue,
                "FEATURE_TYPE_UNSPECIFIED" => FeatureType::FeatureTypeUnspecified,
                "FERRY" => FeatureType::Ferry,
                "FOOTPATH" => FeatureType::Footpath,
                "FOREST" => FeatureType::Forest,
                "HIGHWAY" => FeatureType::Highway,
                "LOCAL_ROAD" => FeatureType::LocalRoad,
                "LOCALITY" => FeatureType::Locality,
                "LODGING" => FeatureType::Lodging,
                "PARK" => FeatureType::Park,
                "POLITICAL" => FeatureType::Political,
                "RAIL" => FeatureType::Rail,
                "REGION" => FeatureType::Region,
                "RESTAURANT" => FeatureType::Restaurant,
                "ROAD" => FeatureType::Road,
                "SCHOOL" => FeatureType::School,
                "SEGMENT" => FeatureType::Segment,
                "SHOPPING" => FeatureType::Shopping,
                "STRUCTURE" => FeatureType::Structure,
                "SUBLOCALITY" => FeatureType::Sublocality,
                "TOURIST_DESTINATION" => FeatureType::TouristDestination,
                "WATER" => FeatureType::Water,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for FeatureType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for FeatureType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FeatureType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ADMINISTRATIVE_AREA1" => FeatureType::AdministrativeArea1,
                "ARTERIAL_ROAD" => FeatureType::ArterialRoad,
                "BANK" => FeatureType::Bank,
                "BAR" => FeatureType::Bar,
                "BEACH" => FeatureType::Beach,
                "CAFE" => FeatureType::Cafe,
                "CONTROLLED_ACCESS_HIGHWAY" => FeatureType::ControlledAccessHighway,
                "EVENT_VENUE" => FeatureType::EventVenue,
                "FEATURE_TYPE_UNSPECIFIED" => FeatureType::FeatureTypeUnspecified,
                "FERRY" => FeatureType::Ferry,
                "FOOTPATH" => FeatureType::Footpath,
                "FOREST" => FeatureType::Forest,
                "HIGHWAY" => FeatureType::Highway,
                "LOCAL_ROAD" => FeatureType::LocalRoad,
                "LOCALITY" => FeatureType::Locality,
                "LODGING" => FeatureType::Lodging,
                "PARK" => FeatureType::Park,
                "POLITICAL" => FeatureType::Political,
                "RAIL" => FeatureType::Rail,
                "REGION" => FeatureType::Region,
                "RESTAURANT" => FeatureType::Restaurant,
                "ROAD" => FeatureType::Road,
                "SCHOOL" => FeatureType::School,
                "SEGMENT" => FeatureType::Segment,
                "SHOPPING" => FeatureType::Shopping,
                "STRUCTURE" => FeatureType::Structure,
                "SUBLOCALITY" => FeatureType::Sublocality,
                "TOURIST_DESTINATION" => FeatureType::TouristDestination,
                "WATER" => FeatureType::Water,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for FeatureType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FeatureType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct FeatureTile {
        #[doc = "The global tile coordinates that uniquely identify this tile."]
        #[serde(
            rename = "coordinates",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub coordinates: ::std::option::Option<crate::schemas::TileCoordinates>,
        #[doc = "Features present on this map tile."]
        #[serde(
            rename = "features",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub features: ::std::option::Option<Vec<crate::schemas::Feature>>,
        #[doc = "Resource name of the tile. The tile resource name is prefixed by its collection ID `tiles/` followed by the resource ID, which encodes the tile's global x and y coordinates and zoom level as `@,,z`. For example, `tiles/@1,2,3z`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Data providers for the data contained in this tile."]
        #[serde(
            rename = "providers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub providers: ::std::option::Option<Vec<crate::schemas::ProviderInfo>>,
        #[doc = "Tile response status code to support tile caching."]
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<crate::schemas::FeatureTileStatus>,
        #[doc = "An opaque value, usually less than 30 characters, that contains version info about this tile and the data that was used to generate it. The client should store this value in its tile cache and pass it back to the API in the client_tile_version_id field of subsequent tile requests in order to enable the API to detect when the new tile would be the same as the one the client already has in its cache. Also see STATUS_OK_DATA_UNCHANGED."]
        #[serde(
            rename = "versionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for FeatureTile {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FeatureTile {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum FeatureTileStatus {
        #[doc = "Everything worked out OK. The cache-control header determines how long this Tile response may be cached by the client. See also version_id and STATUS_OK_DATA_UNCHANGED."]
        StatusOk,
        #[doc = "Indicates that the request was processed successfully and that the tile data that would have been returned are identical to the data already in the client's cache, as specified by the value of client_tile_version_id contained in GetFeatureTileRequest. In particular, the tile's features and providers will not be populated when the tile data is identical. However, the cache-control header and version_id can still change even when the tile contents itself does not, so clients should always use the most recent values returned by the API."]
        StatusOkDataUnchanged,
    }
    impl FeatureTileStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                FeatureTileStatus::StatusOk => "STATUS_OK",
                FeatureTileStatus::StatusOkDataUnchanged => "STATUS_OK_DATA_UNCHANGED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for FeatureTileStatus {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for FeatureTileStatus {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<FeatureTileStatus, ()> {
            Ok(match s {
                "STATUS_OK" => FeatureTileStatus::StatusOk,
                "STATUS_OK_DATA_UNCHANGED" => FeatureTileStatus::StatusOkDataUnchanged,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for FeatureTileStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for FeatureTileStatus {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FeatureTileStatus {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "STATUS_OK" => FeatureTileStatus::StatusOk,
                "STATUS_OK_DATA_UNCHANGED" => FeatureTileStatus::StatusOkDataUnchanged,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for FeatureTileStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FeatureTileStatus {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct FirstDerivativeElevationGrid {
        #[doc = "A multiplier applied to the altitude fields below to extract the actual altitudes in meters from the elevation grid."]
        #[serde(
            rename = "altitudeMultiplier",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub altitude_multiplier: ::std::option::Option<f32>,
        #[doc = "Rows of points containing altitude data making up the elevation grid. Each row is the same length. Rows are ordered from north to south. E.g: rows[0] is the north-most row, and rows[n] is the south-most row."]
        #[serde(
            rename = "rows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rows: ::std::option::Option<Vec<crate::schemas::Row>>,
    }
    impl ::google_field_selector::FieldSelector for FirstDerivativeElevationGrid {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FirstDerivativeElevationGrid {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Geometry {
        #[doc = "The areas present in this geometry."]
        #[serde(
            rename = "areas",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub areas: ::std::option::Option<Vec<crate::schemas::Area>>,
        #[doc = "The extruded areas present in this geometry. Not populated if modeled_volumes are included in this geometry unless always_include_building_footprints is set in GetFeatureTileRequest, in which case the client should decide which (extruded areas or modeled volumes) should be used (they should not be rendered together)."]
        #[serde(
            rename = "extrudedAreas",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub extruded_areas: ::std::option::Option<Vec<crate::schemas::ExtrudedArea>>,
        #[doc = "The lines present in this geometry."]
        #[serde(
            rename = "lines",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub lines: ::std::option::Option<Vec<crate::schemas::Line>>,
        #[doc = "The modeled volumes present in this geometry. Not populated unless enable_modeled_volumes has been set in GetFeatureTileRequest."]
        #[serde(
            rename = "modeledVolumes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub modeled_volumes: ::std::option::Option<Vec<crate::schemas::ModeledVolume>>,
    }
    impl ::google_field_selector::FieldSelector for Geometry {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Geometry {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Line {
        #[doc = "The z-order of this geometry when rendered on a flat basemap. Geometry with a lower z-order should be rendered beneath geometry with a higher z-order. This z-ordering does not imply anything about the altitude of the area relative to the ground, but it can be used to prevent z-fighting. Unlike Line.z_order this can be used to compare with Area.basemap_z_order, and in fact may yield more accurate rendering (where a line may be rendered beneath an area)."]
        #[serde(
            rename = "basemapZOrder",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub basemap_z_order: ::std::option::Option<crate::schemas::BasemapZOrder>,
        #[doc = "The vertices present in the polyline."]
        #[serde(
            rename = "vertexOffsets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub vertex_offsets: ::std::option::Option<crate::schemas::Vertex2DList>,
        #[doc = "The z-order of the line. Lines with a lower z-order should be rendered beneath lines with a higher z-order. This z-ordering does not imply anything about the altitude of the area relative to the ground, but it can be used to prevent z-fighting during rendering on the client. In general, larger and more important road features will have a higher z-order line associated with them. This z-ordering can only be used to compare lines, and cannot be compared with the z_order field in the Area message. The z-order may be negative or zero. Prefer Line.basemap_z_order."]
        #[serde(
            rename = "zOrder",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub z_order: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for Line {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Line {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ModeledVolume {
        #[doc = "The triangle strips present in this mesh."]
        #[serde(
            rename = "strips",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub strips: ::std::option::Option<Vec<crate::schemas::TriangleStrip>>,
        #[doc = "The vertices present in the mesh defining the modeled volume."]
        #[serde(
            rename = "vertexOffsets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub vertex_offsets: ::std::option::Option<crate::schemas::Vertex3DList>,
    }
    impl ::google_field_selector::FieldSelector for ModeledVolume {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ModeledVolume {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ProviderInfo {
        #[doc = "Attribution string for this provider. This string is not localized."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ProviderInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ProviderInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Relation {
        #[doc = "Zero-based index to look up the related feature from the list of features in the tile."]
        #[serde(
            rename = "relatedFeatureIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub related_feature_index: ::std::option::Option<i32>,
        #[doc = "Relation type between the origin feature to the related feature."]
        #[serde(
            rename = "relationType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub relation_type: ::std::option::Option<crate::schemas::RelationRelationType>,
    }
    impl ::google_field_selector::FieldSelector for Relation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Relation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum RelationRelationType {
        #[doc = "The origin feature occupies the related feature."]
        Occupies,
        #[doc = "The origin feature is primarily occupied by the related feature."]
        PrimarilyOccupiedBy,
        #[doc = "Unspecified relation type. Should never happen."]
        RelationTypeUnspecified,
    }
    impl RelationRelationType {
        pub fn as_str(self) -> &'static str {
            match self {
                RelationRelationType::Occupies => "OCCUPIES",
                RelationRelationType::PrimarilyOccupiedBy => "PRIMARILY_OCCUPIED_BY",
                RelationRelationType::RelationTypeUnspecified => "RELATION_TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for RelationRelationType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for RelationRelationType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<RelationRelationType, ()> {
            Ok(match s {
                "OCCUPIES" => RelationRelationType::Occupies,
                "PRIMARILY_OCCUPIED_BY" => RelationRelationType::PrimarilyOccupiedBy,
                "RELATION_TYPE_UNSPECIFIED" => RelationRelationType::RelationTypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for RelationRelationType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for RelationRelationType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RelationRelationType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "OCCUPIES" => RelationRelationType::Occupies,
                "PRIMARILY_OCCUPIED_BY" => RelationRelationType::PrimarilyOccupiedBy,
                "RELATION_TYPE_UNSPECIFIED" => RelationRelationType::RelationTypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for RelationRelationType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RelationRelationType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct RoadInfo {
        #[doc = "Road has signage discouraging or prohibiting use by the general public. E.g., roads with signs that say \"Private\", or \"No trespassing.\""]
        #[serde(
            rename = "isPrivate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_private: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for RoadInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RoadInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Row {
        #[doc = "The difference between each successive pair of altitudes, from west to east. The first, westmost point, is just the altitude rather than a diff. The units are specified by the altitude_multiplier parameter above; the value in meters is given by altitude_multiplier * altitude_diffs[n]. The altitude row (in metres above sea level) can be reconstructed with: a[0] = altitude_diffs[0] * altitude_multiplier when n > 0, a[n] = a[n-1] + altitude_diffs[n-1] * altitude_multiplier."]
        #[serde(
            rename = "altitudeDiffs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub altitude_diffs: ::std::option::Option<Vec<i32>>,
    }
    impl ::google_field_selector::FieldSelector for Row {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Row {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SecondDerivativeElevationGrid {
        #[doc = "A multiplier applied to the elements in the encoded data to extract the actual altitudes in meters."]
        #[serde(
            rename = "altitudeMultiplier",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub altitude_multiplier: ::std::option::Option<f32>,
        #[doc = "The number of columns included in the encoded elevation data (i.e. the horizontal resolution of the grid)."]
        #[serde(
            rename = "columnCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub column_count: ::std::option::Option<i32>,
        #[doc = "A stream of elements each representing a point on the tile running across each row from left to right, top to bottom. There will be precisely horizontal_resolution * vertical_resolution elements in the stream. The elements are not the heights, rather the second order derivative of the values one would expect in a stream of height data. Each element is a varint with the following encoding: ------------------------------------------------------------------------| | Head Nibble | ------------------------------------------------------------------------| | Bit 0 | Bit 1 | Bits 2-3 | | Terminator| Sign (1=neg) | Least significant 2 bits of absolute error | ------------------------------------------------------------------------| | Tail Nibble #1 | ------------------------------------------------------------------------| | Bit 0 | Bit 1-3 | | Terminator| Least significant 3 bits of absolute error | ------------------------------------------------------------------------| | ... | Tail Nibble #n | ------------------------------------------------------------------------| | Bit 0 | Bit 1-3 | | Terminator| Least significant 3 bits of absolute error | ------------------------------------------------------------------------|"]
        #[serde(
            rename = "encodedData",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub encoded_data: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "The number of rows included in the encoded elevation data (i.e. the vertical resolution of the grid)."]
        #[serde(
            rename = "rowCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub row_count: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for SecondDerivativeElevationGrid {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SecondDerivativeElevationGrid {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SegmentInfo {
        #[doc = "Metadata for features with the ROAD FeatureType."]
        #[serde(
            rename = "roadInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub road_info: ::std::option::Option<crate::schemas::RoadInfo>,
    }
    impl ::google_field_selector::FieldSelector for SegmentInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SegmentInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TerrainTile {
        #[doc = "The global tile coordinates that uniquely identify this tile."]
        #[serde(
            rename = "coordinates",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub coordinates: ::std::option::Option<crate::schemas::TileCoordinates>,
        #[doc = "Terrain elevation data encoded as a FirstDerivativeElevationGrid."]
        #[serde(
            rename = "firstDerivative",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub first_derivative: ::std::option::Option<crate::schemas::FirstDerivativeElevationGrid>,
        #[doc = "Resource name of the tile. The tile resource name is prefixed by its collection ID `terrain/` followed by the resource ID, which encodes the tile's global x and y coordinates and zoom level as `@,,z`. For example, `terrain/@1,2,3z`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Terrain elevation data encoded as a SecondDerivativeElevationGrid. ."]
        #[serde(
            rename = "secondDerivative",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub second_derivative: ::std::option::Option<crate::schemas::SecondDerivativeElevationGrid>,
    }
    impl ::google_field_selector::FieldSelector for TerrainTile {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TerrainTile {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TileCoordinates {
        #[doc = "Required. The x coordinate."]
        #[serde(
            rename = "x",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub x: ::std::option::Option<i32>,
        #[doc = "Required. The y coordinate."]
        #[serde(
            rename = "y",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub y: ::std::option::Option<i32>,
        #[doc = "Required. The Google Maps API zoom level."]
        #[serde(
            rename = "zoom",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub zoom: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for TileCoordinates {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TileCoordinates {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TriangleStrip {
        #[doc = "Index into the vertex_offset array representing the next vertex in the triangle strip."]
        #[serde(
            rename = "vertexIndices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub vertex_indices: ::std::option::Option<Vec<i32>>,
    }
    impl ::google_field_selector::FieldSelector for TriangleStrip {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TriangleStrip {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Vertex2DList {
        #[doc = "List of x-offsets in local tile coordinates."]
        #[serde(
            rename = "xOffsets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub x_offsets: ::std::option::Option<Vec<i32>>,
        #[doc = "List of y-offsets in local tile coordinates."]
        #[serde(
            rename = "yOffsets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub y_offsets: ::std::option::Option<Vec<i32>>,
    }
    impl ::google_field_selector::FieldSelector for Vertex2DList {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Vertex2DList {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Vertex3DList {
        #[doc = "List of x-offsets in local tile coordinates."]
        #[serde(
            rename = "xOffsets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub x_offsets: ::std::option::Option<Vec<i32>>,
        #[doc = "List of y-offsets in local tile coordinates."]
        #[serde(
            rename = "yOffsets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub y_offsets: ::std::option::Option<Vec<i32>>,
        #[doc = "List of z-offsets in local tile coordinates."]
        #[serde(
            rename = "zOffsets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub z_offsets: ::std::option::Option<Vec<i32>>,
    }
    impl ::google_field_selector::FieldSelector for Vertex3DList {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Vertex3DList {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
}
pub mod params {
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Alt {
        #[doc = "Responses with Content-Type of application/json"]
        Json,
        #[doc = "Media download with context-dependent Content-Type"]
        Media,
        #[doc = "Responses with Content-Type of application/x-protobuf"]
        Proto,
    }
    impl Alt {
        pub fn as_str(self) -> &'static str {
            match self {
                Alt::Json => "json",
                Alt::Media => "media",
                Alt::Proto => "proto",
            }
        }
    }
    impl ::std::convert::AsRef<str> for Alt {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for Alt {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<Alt, ()> {
            Ok(match s {
                "json" => Alt::Json,
                "media" => Alt::Media,
                "proto" => Alt::Proto,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for Alt {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Alt {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Alt {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "json" => Alt::Json,
                "media" => Alt::Media,
                "proto" => Alt::Proto,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for Alt {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Alt {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Xgafv {
        #[doc = "v1 error format"]
        _1,
        #[doc = "v2 error format"]
        _2,
    }
    impl Xgafv {
        pub fn as_str(self) -> &'static str {
            match self {
                Xgafv::_1 => "1",
                Xgafv::_2 => "2",
            }
        }
    }
    impl ::std::convert::AsRef<str> for Xgafv {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for Xgafv {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<Xgafv, ()> {
            Ok(match s {
                "1" => Xgafv::_1,
                "2" => Xgafv::_2,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for Xgafv {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Xgafv {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Xgafv {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "1" => Xgafv::_1,
                "2" => Xgafv::_2,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for Xgafv {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Xgafv {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
}
pub struct Client {
    reqwest: ::reqwest::blocking::Client,
    auth: Box<dyn ::google_api_auth::GetAccessToken>,
}
impl Client {
    pub fn new<A>(auth: A) -> Self
    where
        A: ::google_api_auth::GetAccessToken + 'static,
    {
        Client::with_reqwest_client(
            auth,
            ::reqwest::blocking::Client::builder()
                .timeout(None)
                .build()
                .unwrap(),
        )
    }
    pub fn with_reqwest_client<A>(auth: A, reqwest: ::reqwest::blocking::Client) -> Self
    where
        A: ::google_api_auth::GetAccessToken + 'static,
    {
        Client {
            reqwest,
            auth: Box::new(auth),
        }
    }
    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
        self.auth.as_ref()
    }
    #[doc = "Actions that can be performed on the featuretiles resource"]
    pub fn featuretiles(&self) -> crate::resources::featuretiles::FeaturetilesActions {
        crate::resources::featuretiles::FeaturetilesActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the terraintiles resource"]
    pub fn terraintiles(&self) -> crate::resources::terraintiles::TerraintilesActions {
        crate::resources::terraintiles::TerraintilesActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod featuretiles {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum GetClientInfoPlatform {
                #[doc = "Android"]
                Android,
                #[doc = "Development environment."]
                Editor,
                #[doc = "iOS"]
                Ios,
                #[doc = "Linux"]
                Linux,
                #[doc = "macOS."]
                MacOs,
                #[doc = "Unspecified or unknown OS."]
                PlatformUnspecified,
                #[doc = "WebGL."]
                WebGl,
                #[doc = "Windows."]
                Windows,
            }
            impl GetClientInfoPlatform {
                pub fn as_str(self) -> &'static str {
                    match self {
                        GetClientInfoPlatform::Android => "ANDROID",
                        GetClientInfoPlatform::Editor => "EDITOR",
                        GetClientInfoPlatform::Ios => "IOS",
                        GetClientInfoPlatform::Linux => "LINUX",
                        GetClientInfoPlatform::MacOs => "MAC_OS",
                        GetClientInfoPlatform::PlatformUnspecified => "PLATFORM_UNSPECIFIED",
                        GetClientInfoPlatform::WebGl => "WEB_GL",
                        GetClientInfoPlatform::Windows => "WINDOWS",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for GetClientInfoPlatform {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for GetClientInfoPlatform {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<GetClientInfoPlatform, ()> {
                    Ok(match s {
                        "ANDROID" => GetClientInfoPlatform::Android,
                        "EDITOR" => GetClientInfoPlatform::Editor,
                        "IOS" => GetClientInfoPlatform::Ios,
                        "LINUX" => GetClientInfoPlatform::Linux,
                        "MAC_OS" => GetClientInfoPlatform::MacOs,
                        "PLATFORM_UNSPECIFIED" => GetClientInfoPlatform::PlatformUnspecified,
                        "WEB_GL" => GetClientInfoPlatform::WebGl,
                        "WINDOWS" => GetClientInfoPlatform::Windows,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for GetClientInfoPlatform {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for GetClientInfoPlatform {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for GetClientInfoPlatform {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "ANDROID" => GetClientInfoPlatform::Android,
                        "EDITOR" => GetClientInfoPlatform::Editor,
                        "IOS" => GetClientInfoPlatform::Ios,
                        "LINUX" => GetClientInfoPlatform::Linux,
                        "MAC_OS" => GetClientInfoPlatform::MacOs,
                        "PLATFORM_UNSPECIFIED" => GetClientInfoPlatform::PlatformUnspecified,
                        "WEB_GL" => GetClientInfoPlatform::WebGl,
                        "WINDOWS" => GetClientInfoPlatform::Windows,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for GetClientInfoPlatform {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for GetClientInfoPlatform {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
        }
        pub struct FeaturetilesActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> FeaturetilesActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Gets a feature tile by its tile resource name."]
            pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    name: name.into(),
                    always_include_building_footprints: None,
                    client_info_api_client: None,
                    client_info_application_id: None,
                    client_info_application_version: None,
                    client_info_device_model: None,
                    client_info_operating_system: None,
                    client_info_platform: None,
                    client_info_user_id: None,
                    client_tile_version_id: None,
                    enable_detailed_highway_types: None,
                    enable_feature_names: None,
                    enable_modeled_volumes: None,
                    enable_political_features: None,
                    enable_private_roads: None,
                    enable_unclipped_buildings: None,
                    language_code: None,
                    region_code: None,
                }
            }
        }
        #[doc = "Created via [FeaturetilesActions::get()](struct.FeaturetilesActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            name: String,
            always_include_building_footprints: Option<bool>,
            client_info_api_client: Option<String>,
            client_info_application_id: Option<String>,
            client_info_application_version: Option<String>,
            client_info_device_model: Option<String>,
            client_info_operating_system: Option<String>,
            client_info_platform:
                Option<crate::resources::featuretiles::params::GetClientInfoPlatform>,
            client_info_user_id: Option<String>,
            client_tile_version_id: Option<String>,
            enable_detailed_highway_types: Option<bool>,
            enable_feature_names: Option<bool>,
            enable_modeled_volumes: Option<bool>,
            enable_political_features: Option<bool>,
            enable_private_roads: Option<bool>,
            enable_unclipped_buildings: Option<bool>,
            language_code: Option<String>,
            region_code: Option<String>,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> GetRequestBuilder<'a> {
            #[doc = "Flag indicating whether the returned tile will always contain 2.5D footprints for structures. If enabled_modeled_volumes is set, this will mean that structures will have both their 3D models and 2.5D footprints returned."]
            pub fn always_include_building_footprints(mut self, value: bool) -> Self {
                self.always_include_building_footprints = Some(value);
                self
            }
            #[doc = "API client name and version. For example, the SDK calling the API. The exact format is up to the client."]
            pub fn client_info_api_client(mut self, value: impl Into<String>) -> Self {
                self.client_info_api_client = Some(value.into());
                self
            }
            #[doc = "Application ID, such as the package name on Android and the bundle identifier on iOS platforms."]
            pub fn client_info_application_id(mut self, value: impl Into<String>) -> Self {
                self.client_info_application_id = Some(value.into());
                self
            }
            #[doc = "Application version number, such as \"1.2.3\". The exact format is application-dependent."]
            pub fn client_info_application_version(mut self, value: impl Into<String>) -> Self {
                self.client_info_application_version = Some(value.into());
                self
            }
            #[doc = "Device model as reported by the device. The exact format is platform-dependent."]
            pub fn client_info_device_model(mut self, value: impl Into<String>) -> Self {
                self.client_info_device_model = Some(value.into());
                self
            }
            #[doc = "Operating system name and version as reported by the OS. For example, \"Mac OS X 10.10.4\". The exact format is platform-dependent."]
            pub fn client_info_operating_system(mut self, value: impl Into<String>) -> Self {
                self.client_info_operating_system = Some(value.into());
                self
            }
            #[doc = "Platform where the application is running."]
            pub fn client_info_platform(
                mut self,
                value: crate::resources::featuretiles::params::GetClientInfoPlatform,
            ) -> Self {
                self.client_info_platform = Some(value);
                self
            }
            #[doc = "Required. A client-generated user ID. The ID should be generated and persisted during the first user session or whenever a pre-existing ID is not found. The exact format is up to the client. This must be non-empty in a GetFeatureTileRequest (whether via the header or GetFeatureTileRequest.client_info)."]
            pub fn client_info_user_id(mut self, value: impl Into<String>) -> Self {
                self.client_info_user_id = Some(value.into());
                self
            }
            #[doc = "Optional version id identifying the tile that is already in the client's cache. This field should be populated with the most recent version_id value returned by the API for the requested tile. If the version id is empty the server always returns a newly rendered tile. If it is provided the server checks if the tile contents would be identical to one that's already on the client, and if so, returns a stripped-down response tile with STATUS_OK_DATA_UNCHANGED instead."]
            pub fn client_tile_version_id(mut self, value: impl Into<String>) -> Self {
                self.client_tile_version_id = Some(value.into());
                self
            }
            #[doc = "Flag indicating whether detailed highway types should be returned. If this is set, the CONTROLLED_ACCESS_HIGHWAY type may be returned. If not, then these highways will have the generic HIGHWAY type. This exists for backwards compatibility reasons."]
            pub fn enable_detailed_highway_types(mut self, value: bool) -> Self {
                self.enable_detailed_highway_types = Some(value);
                self
            }
            #[doc = "Flag indicating whether human-readable names should be returned for features. If this is set, the display_name field on the feature will be filled out."]
            pub fn enable_feature_names(mut self, value: bool) -> Self {
                self.enable_feature_names = Some(value);
                self
            }
            #[doc = "Flag indicating whether 3D building models should be enabled. If this is set structures will be returned as 3D modeled volumes rather than 2.5D extruded areas where possible."]
            pub fn enable_modeled_volumes(mut self, value: bool) -> Self {
                self.enable_modeled_volumes = Some(value);
                self
            }
            #[doc = "Flag indicating whether political features should be returned."]
            pub fn enable_political_features(mut self, value: bool) -> Self {
                self.enable_political_features = Some(value);
                self
            }
            #[doc = "Flag indicating whether the returned tile will contain road features that are marked private. Private roads are indicated by the Feature.segment_info.road_info.is_private field."]
            pub fn enable_private_roads(mut self, value: bool) -> Self {
                self.enable_private_roads = Some(value);
                self
            }
            #[doc = "Flag indicating whether unclipped buildings should be returned. If this is set, building render ops will extend beyond the tile boundary. Buildings will only be returned on the tile that contains their centroid."]
            pub fn enable_unclipped_buildings(mut self, value: bool) -> Self {
                self.enable_unclipped_buildings = Some(value);
                self
            }
            #[doc = "Required. The BCP-47 language code corresponding to the language in which the name was requested, such as \"en-US\" or \"sr-Latn\". For more information, see http://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
            pub fn language_code(mut self, value: impl Into<String>) -> Self {
                self.language_code = Some(value.into());
                self
            }
            #[doc = "Required. The Unicode country/region code (CLDR) of the location from which the request is coming from, such as \"US\" and \"419\". For more information, see http://www.unicode.org/reports/tr35/#unicode_region_subtag."]
            pub fn region_code(mut self, value: impl Into<String>) -> Self {
                self.region_code = Some(value.into());
                self
            }
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(mut self, value: impl Into<String>) -> Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(mut self, value: bool) -> Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                self.xgafv = Some(value);
                self
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::FeatureTile, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::FeatureTile, crate::Error> {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(mut self, fields: Option<F>) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path())?;
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://vectortile.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[(
                    "alwaysIncludeBuildingFootprints",
                    &self.always_include_building_footprints,
                )]);
                req = req.query(&[("clientInfo.apiClient", &self.client_info_api_client)]);
                req = req.query(&[("clientInfo.applicationId", &self.client_info_application_id)]);
                req = req.query(&[(
                    "clientInfo.applicationVersion",
                    &self.client_info_application_version,
                )]);
                req = req.query(&[("clientInfo.deviceModel", &self.client_info_device_model)]);
                req = req.query(&[(
                    "clientInfo.operatingSystem",
                    &self.client_info_operating_system,
                )]);
                req = req.query(&[("clientInfo.platform", &self.client_info_platform)]);
                req = req.query(&[("clientInfo.userId", &self.client_info_user_id)]);
                req = req.query(&[("clientTileVersionId", &self.client_tile_version_id)]);
                req = req.query(&[(
                    "enableDetailedHighwayTypes",
                    &self.enable_detailed_highway_types,
                )]);
                req = req.query(&[("enableFeatureNames", &self.enable_feature_names)]);
                req = req.query(&[("enableModeledVolumes", &self.enable_modeled_volumes)]);
                req = req.query(&[("enablePoliticalFeatures", &self.enable_political_features)]);
                req = req.query(&[("enablePrivateRoads", &self.enable_private_roads)]);
                req = req.query(&[("enableUnclippedBuildings", &self.enable_unclipped_buildings)]);
                req = req.query(&[("languageCode", &self.language_code)]);
                req = req.query(&[("regionCode", &self.region_code)]);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
    }
    pub mod terraintiles {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum GetClientInfoPlatform {
                #[doc = "Android"]
                Android,
                #[doc = "Development environment."]
                Editor,
                #[doc = "iOS"]
                Ios,
                #[doc = "Linux"]
                Linux,
                #[doc = "macOS."]
                MacOs,
                #[doc = "Unspecified or unknown OS."]
                PlatformUnspecified,
                #[doc = "WebGL."]
                WebGl,
                #[doc = "Windows."]
                Windows,
            }
            impl GetClientInfoPlatform {
                pub fn as_str(self) -> &'static str {
                    match self {
                        GetClientInfoPlatform::Android => "ANDROID",
                        GetClientInfoPlatform::Editor => "EDITOR",
                        GetClientInfoPlatform::Ios => "IOS",
                        GetClientInfoPlatform::Linux => "LINUX",
                        GetClientInfoPlatform::MacOs => "MAC_OS",
                        GetClientInfoPlatform::PlatformUnspecified => "PLATFORM_UNSPECIFIED",
                        GetClientInfoPlatform::WebGl => "WEB_GL",
                        GetClientInfoPlatform::Windows => "WINDOWS",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for GetClientInfoPlatform {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for GetClientInfoPlatform {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<GetClientInfoPlatform, ()> {
                    Ok(match s {
                        "ANDROID" => GetClientInfoPlatform::Android,
                        "EDITOR" => GetClientInfoPlatform::Editor,
                        "IOS" => GetClientInfoPlatform::Ios,
                        "LINUX" => GetClientInfoPlatform::Linux,
                        "MAC_OS" => GetClientInfoPlatform::MacOs,
                        "PLATFORM_UNSPECIFIED" => GetClientInfoPlatform::PlatformUnspecified,
                        "WEB_GL" => GetClientInfoPlatform::WebGl,
                        "WINDOWS" => GetClientInfoPlatform::Windows,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for GetClientInfoPlatform {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for GetClientInfoPlatform {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for GetClientInfoPlatform {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "ANDROID" => GetClientInfoPlatform::Android,
                        "EDITOR" => GetClientInfoPlatform::Editor,
                        "IOS" => GetClientInfoPlatform::Ios,
                        "LINUX" => GetClientInfoPlatform::Linux,
                        "MAC_OS" => GetClientInfoPlatform::MacOs,
                        "PLATFORM_UNSPECIFIED" => GetClientInfoPlatform::PlatformUnspecified,
                        "WEB_GL" => GetClientInfoPlatform::WebGl,
                        "WINDOWS" => GetClientInfoPlatform::Windows,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for GetClientInfoPlatform {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for GetClientInfoPlatform {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum GetTerrainFormatsItems {
                #[doc = "Terrain elevation data encoded as a FirstDerivativeElevationGrid. ."]
                FirstDerivative,
                #[doc = "Terrain elevation data encoded as a SecondDerivativeElevationGrid."]
                SecondDerivative,
                #[doc = "An unknown or unspecified terrain format."]
                TerrainFormatUnknown,
            }
            impl GetTerrainFormatsItems {
                pub fn as_str(self) -> &'static str {
                    match self {
                        GetTerrainFormatsItems::FirstDerivative => "FIRST_DERIVATIVE",
                        GetTerrainFormatsItems::SecondDerivative => "SECOND_DERIVATIVE",
                        GetTerrainFormatsItems::TerrainFormatUnknown => "TERRAIN_FORMAT_UNKNOWN",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for GetTerrainFormatsItems {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for GetTerrainFormatsItems {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<GetTerrainFormatsItems, ()> {
                    Ok(match s {
                        "FIRST_DERIVATIVE" => GetTerrainFormatsItems::FirstDerivative,
                        "SECOND_DERIVATIVE" => GetTerrainFormatsItems::SecondDerivative,
                        "TERRAIN_FORMAT_UNKNOWN" => GetTerrainFormatsItems::TerrainFormatUnknown,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for GetTerrainFormatsItems {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for GetTerrainFormatsItems {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for GetTerrainFormatsItems {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "FIRST_DERIVATIVE" => GetTerrainFormatsItems::FirstDerivative,
                        "SECOND_DERIVATIVE" => GetTerrainFormatsItems::SecondDerivative,
                        "TERRAIN_FORMAT_UNKNOWN" => GetTerrainFormatsItems::TerrainFormatUnknown,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for GetTerrainFormatsItems {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for GetTerrainFormatsItems {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
        }
        pub struct TerraintilesActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> TerraintilesActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Gets a terrain tile by its tile resource name."]
            pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    name: name.into(),
                    altitude_precision_centimeters: None,
                    client_info_api_client: None,
                    client_info_application_id: None,
                    client_info_application_version: None,
                    client_info_device_model: None,
                    client_info_operating_system: None,
                    client_info_platform: None,
                    client_info_user_id: None,
                    max_elevation_resolution_cells: None,
                    min_elevation_resolution_cells: None,
                    terrain_formats: None,
                }
            }
        }
        #[doc = "Created via [TerraintilesActions::get()](struct.TerraintilesActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            name: String,
            altitude_precision_centimeters: Option<i32>,
            client_info_api_client: Option<String>,
            client_info_application_id: Option<String>,
            client_info_application_version: Option<String>,
            client_info_device_model: Option<String>,
            client_info_operating_system: Option<String>,
            client_info_platform:
                Option<crate::resources::terraintiles::params::GetClientInfoPlatform>,
            client_info_user_id: Option<String>,
            max_elevation_resolution_cells: Option<i32>,
            min_elevation_resolution_cells: Option<i32>,
            terrain_formats:
                Option<Vec<crate::resources::terraintiles::params::GetTerrainFormatsItems>>,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> GetRequestBuilder<'a> {
            #[doc = "The precision of terrain altitudes in centimeters. Possible values: between 1 (cm level precision) and 1,000,000 (10-kilometer level precision)."]
            pub fn altitude_precision_centimeters(mut self, value: i32) -> Self {
                self.altitude_precision_centimeters = Some(value);
                self
            }
            #[doc = "API client name and version. For example, the SDK calling the API. The exact format is up to the client."]
            pub fn client_info_api_client(mut self, value: impl Into<String>) -> Self {
                self.client_info_api_client = Some(value.into());
                self
            }
            #[doc = "Application ID, such as the package name on Android and the bundle identifier on iOS platforms."]
            pub fn client_info_application_id(mut self, value: impl Into<String>) -> Self {
                self.client_info_application_id = Some(value.into());
                self
            }
            #[doc = "Application version number, such as \"1.2.3\". The exact format is application-dependent."]
            pub fn client_info_application_version(mut self, value: impl Into<String>) -> Self {
                self.client_info_application_version = Some(value.into());
                self
            }
            #[doc = "Device model as reported by the device. The exact format is platform-dependent."]
            pub fn client_info_device_model(mut self, value: impl Into<String>) -> Self {
                self.client_info_device_model = Some(value.into());
                self
            }
            #[doc = "Operating system name and version as reported by the OS. For example, \"Mac OS X 10.10.4\". The exact format is platform-dependent."]
            pub fn client_info_operating_system(mut self, value: impl Into<String>) -> Self {
                self.client_info_operating_system = Some(value.into());
                self
            }
            #[doc = "Platform where the application is running."]
            pub fn client_info_platform(
                mut self,
                value: crate::resources::terraintiles::params::GetClientInfoPlatform,
            ) -> Self {
                self.client_info_platform = Some(value);
                self
            }
            #[doc = "Required. A client-generated user ID. The ID should be generated and persisted during the first user session or whenever a pre-existing ID is not found. The exact format is up to the client. This must be non-empty in a GetFeatureTileRequest (whether via the header or GetFeatureTileRequest.client_info)."]
            pub fn client_info_user_id(mut self, value: impl Into<String>) -> Self {
                self.client_info_user_id = Some(value.into());
                self
            }
            #[doc = "The maximum allowed resolution for the returned elevation heightmap. Possible values: between 1 and 1024 (and not less than min_elevation_resolution_cells). Over-sized heightmaps will be non-uniformly down-sampled such that each edge is no longer than this value. Non-uniformity is chosen to maximise the amount of preserved data. For example: Original resolution: 100px (width) * 30px (height) max_elevation_resolution: 30 New resolution: 30px (width) * 30px (height)"]
            pub fn max_elevation_resolution_cells(mut self, value: i32) -> Self {
                self.max_elevation_resolution_cells = Some(value);
                self
            }
            #[doc = "The minimum allowed resolution for the returned elevation heightmap. Possible values: between 0 and 1024 (and not more than max_elevation_resolution_cells). Zero is supported for backward compatibility. Under-sized heightmaps will be non-uniformly up-sampled such that each edge is no shorter than this value. Non-uniformity is chosen to maximise the amount of preserved data. For example: Original resolution: 30px (width) * 10px (height) min_elevation_resolution: 30 New resolution: 30px (width) * 30px (height)"]
            pub fn min_elevation_resolution_cells(mut self, value: i32) -> Self {
                self.min_elevation_resolution_cells = Some(value);
                self
            }
            #[doc = "Terrain formats that the client understands."]
            pub fn terrain_formats(
                mut self,
                value: impl Into<Vec<crate::resources::terraintiles::params::GetTerrainFormatsItems>>,
            ) -> Self {
                self.terrain_formats = Some(value.into());
                self
            }
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(mut self, value: impl Into<String>) -> Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(mut self, value: bool) -> Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                self.xgafv = Some(value);
                self
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::TerrainTile, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::TerrainTile, crate::Error> {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(mut self, fields: Option<F>) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path())?;
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://vectortile.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[(
                    "altitudePrecisionCentimeters",
                    &self.altitude_precision_centimeters,
                )]);
                req = req.query(&[("clientInfo.apiClient", &self.client_info_api_client)]);
                req = req.query(&[("clientInfo.applicationId", &self.client_info_application_id)]);
                req = req.query(&[(
                    "clientInfo.applicationVersion",
                    &self.client_info_application_version,
                )]);
                req = req.query(&[("clientInfo.deviceModel", &self.client_info_device_model)]);
                req = req.query(&[(
                    "clientInfo.operatingSystem",
                    &self.client_info_operating_system,
                )]);
                req = req.query(&[("clientInfo.platform", &self.client_info_platform)]);
                req = req.query(&[("clientInfo.userId", &self.client_info_user_id)]);
                req = req.query(&[(
                    "maxElevationResolutionCells",
                    &self.max_elevation_resolution_cells,
                )]);
                req = req.query(&[(
                    "minElevationResolutionCells",
                    &self.min_elevation_resolution_cells,
                )]);
                for value in self.terrain_formats.iter().flatten() {
                    req = req.query(&[("terrainFormats", value)]);
                }
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
    }
}
#[derive(Debug)]
pub enum Error {
    OAuth2(Box<dyn ::std::error::Error + Send + Sync>),
    JSON(::serde_json::Error),
    Reqwest {
        reqwest_err: ::reqwest::Error,
        body: Option<String>,
    },
    Other(Box<dyn ::std::error::Error + Send + Sync>),
}

impl Error {
    pub fn json_error(&self) -> Option<&::serde_json::Error> {
        match self {
            Error::OAuth2(_) => None,
            Error::JSON(err) => Some(err),
            Error::Reqwest { .. } => None,
            Error::Other(_) => None,
        }
    }
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            Error::OAuth2(err) => write!(f, "OAuth2 Error: {}", err),
            Error::JSON(err) => write!(f, "JSON Error: {}", err),
            Error::Reqwest { reqwest_err, body } => {
                write!(f, "Reqwest Error: {}", reqwest_err)?;
                if let Some(body) = body {
                    write!(f, ": {}", body)?;
                }
                Ok(())
            }
            Error::Other(err) => write!(f, "Uknown Error: {}", err),
        }
    }
}

impl ::std::error::Error for Error {}

impl From<::serde_json::Error> for Error {
    fn from(err: ::serde_json::Error) -> Error {
        Error::JSON(err)
    }
}

impl From<::reqwest::Error> for Error {
    fn from(reqwest_err: ::reqwest::Error) -> Error {
        Error::Reqwest {
            reqwest_err,
            body: None,
        }
    }
}

/// Check the response to see if the status code represents an error. If so
/// convert it into the Reqwest variant of Error.
fn error_from_response(
    response: ::reqwest::blocking::Response,
) -> Result<::reqwest::blocking::Response, Error> {
    match response.error_for_status_ref() {
        Err(reqwest_err) => {
            let body = response.text().ok();
            Err(Error::Reqwest { reqwest_err, body })
        }
        Ok(_) => Ok(response),
    }
}
#[allow(dead_code)]
const SIMPLE: &::percent_encoding::AsciiSet = &::percent_encoding::NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'.')
    .remove(b'_')
    .remove(b'~');

#[allow(dead_code)]
const RESERVED: &::percent_encoding::AsciiSet = &SIMPLE
    .remove(b'%')
    .remove(b':')
    .remove(b'/')
    .remove(b'?')
    .remove(b'#')
    .remove(b'[')
    .remove(b']')
    .remove(b'@')
    .remove(b'!')
    .remove(b'$')
    .remove(b'&')
    .remove(b'\'')
    .remove(b'(')
    .remove(b')')
    .remove(b'*')
    .remove(b'+')
    .remove(b',')
    .remove(b';')
    .remove(b'=');
#[allow(dead_code)]
mod multipart {
    pub(crate) struct RelatedMultiPart {
        parts: Vec<Part>,
        boundary: String,
    }

    impl RelatedMultiPart {
        pub(crate) fn new() -> Self {
            RelatedMultiPart {
                parts: Vec::new(),
                boundary: ::textnonce::TextNonce::sized(68).unwrap().0,
            }
        }

        pub(crate) fn new_part(&mut self, part: Part) {
            self.parts.push(part);
        }

        pub(crate) fn boundary(&self) -> &str {
            &self.boundary
        }

        pub(crate) fn into_reader(self) -> RelatedMultiPartReader {
            let boundary_marker = boundary_marker(&self.boundary);
            RelatedMultiPartReader {
                state: RelatedMultiPartReaderState::WriteBoundary {
                    start: 0,
                    boundary: format!("{}\r\n", &boundary_marker),
                },
                boundary: boundary_marker,
                next_body: None,
                parts: self.parts.into_iter(),
            }
        }
    }

    pub(crate) struct Part {
        content_type: ::mime::Mime,
        body: Box<dyn ::std::io::Read + Send>,
    }

    impl Part {
        pub(crate) fn new(
            content_type: ::mime::Mime,
            body: Box<dyn ::std::io::Read + Send>,
        ) -> Part {
            Part { content_type, body }
        }
    }

    pub(crate) struct RelatedMultiPartReader {
        state: RelatedMultiPartReaderState,
        boundary: String,
        next_body: Option<Box<dyn ::std::io::Read + Send>>,
        parts: std::vec::IntoIter<Part>,
    }

    enum RelatedMultiPartReaderState {
        WriteBoundary {
            start: usize,
            boundary: String,
        },
        WriteContentType {
            start: usize,
            content_type: Vec<u8>,
        },
        WriteBody {
            body: Box<dyn ::std::io::Read + Send>,
        },
    }

    impl ::std::io::Read for RelatedMultiPartReader {
        fn read(&mut self, buf: &mut [u8]) -> ::std::io::Result<usize> {
            use RelatedMultiPartReaderState::*;
            let mut bytes_written: usize = 0;
            loop {
                let rem_buf = &mut buf[bytes_written..];
                match &mut self.state {
                    WriteBoundary { start, boundary } => {
                        let bytes_to_copy = std::cmp::min(boundary.len() - *start, rem_buf.len());
                        rem_buf[..bytes_to_copy]
                            .copy_from_slice(&boundary.as_bytes()[*start..*start + bytes_to_copy]);
                        *start += bytes_to_copy;
                        bytes_written += bytes_to_copy;
                        if *start == boundary.len() {
                            let next_part = match self.parts.next() {
                                None => break,
                                Some(part) => part,
                            };
                            self.next_body = Some(next_part.body);
                            self.state = WriteContentType {
                                start: 0,
                                content_type: format!(
                                    "Content-Type: {}\r\n\r\n",
                                    next_part.content_type
                                )
                                .into_bytes(),
                            };
                        } else {
                            break;
                        }
                    }
                    WriteContentType {
                        start,
                        content_type,
                    } => {
                        let bytes_to_copy =
                            std::cmp::min(content_type.len() - *start, rem_buf.len());
                        rem_buf[..bytes_to_copy]
                            .copy_from_slice(&content_type[*start..*start + bytes_to_copy]);
                        *start += bytes_to_copy;
                        bytes_written += bytes_to_copy;
                        if *start == content_type.len() {
                            self.state = WriteBody {
                                body: self.next_body.take().unwrap(),
                            };
                        } else {
                            break;
                        }
                    }
                    WriteBody { body } => {
                        let written = body.read(rem_buf)?;
                        bytes_written += written;
                        if written == 0 {
                            self.state = WriteBoundary {
                                start: 0,
                                boundary: format!("\r\n{}\r\n", &self.boundary),
                            };
                        } else {
                            break;
                        }
                    }
                }
            }
            Ok(bytes_written)
        }
    }

    fn boundary_marker(boundary: &str) -> String {
        let mut marker = String::with_capacity(boundary.len() + 2);
        marker.push_str("--");
        marker.push_str(boundary);
        marker
    }
}
// A serde helper module that can be used with the `with` attribute
// to deserialize any string to a FromStr type and serialize any
// Display type to a String. Google API's encode i64, u64 values as
// strings.
#[allow(dead_code)]
mod parsed_string {
    pub fn serialize<T, S>(
        value: &Option<T>,
        serializer: S,
    ) -> ::std::result::Result<S::Ok, S::Error>
    where
        T: ::std::fmt::Display,
        S: ::serde::Serializer,
    {
        use ::serde::Serialize;
        value.as_ref().map(|x| x.to_string()).serialize(serializer)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> ::std::result::Result<Option<T>, D::Error>
    where
        T: ::std::str::FromStr,
        T::Err: ::std::fmt::Display,
        D: ::serde::de::Deserializer<'de>,
    {
        use ::serde::Deserialize;
        match Option::<String>::deserialize(deserializer)? {
            Some(x) => Ok(Some(x.parse().map_err(::serde::de::Error::custom)?)),
            None => Ok(None),
        }
    }
}
