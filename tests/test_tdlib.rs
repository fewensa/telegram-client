


#[test]
fn test_from_update_user() {
  let json = r#"
{
	"@type": "updateUser",
	"user": {
		"@type": "user",
		"id": 743550508,
		"first_name": "Jo",
		"last_name": "ho",
		"username": "laocaimi",
		"phone_number": "",
		"status": {
			"@type": "userStatusOffline",
			"was_online": 1556077825
		},
		"profile_photo": {
			"@type": "profilePhoto",
			"id": "3193525115240425385",
			"small": {
				"@type": "file",
				"id": 1,
				"size": 0,
				"expected_size": 0,
				"local": {
					"@type": "localFile",
					"path": "",
					"can_be_downloaded": true,
					"can_be_deleted": false,
					"is_downloading_active": false,
					"is_downloading_completed": false,
					"download_offset": 0,
					"downloaded_prefix_size": 0,
					"downloaded_size": 0
				},
				"remote": {
					"@type": "remoteFile",
					"id": "AQADAQADqacxGyyuUSwACL8DCzAABFCmbbpR8R0fdAoDAAEC",
					"is_uploading_active": false,
					"is_uploading_completed": true,
					"uploaded_size": 0
				}
			},
			"big": {
				"@type": "file",
				"id": 2,
				"size": 0,
				"expected_size": 0,
				"local": {
					"@type": "localFile",
					"path": "",
					"can_be_downloaded": true,
					"can_be_deleted": false,
					"is_downloading_active": false,
					"is_downloading_completed": false,
					"download_offset": 0,
					"downloaded_prefix_size": 0,
					"downloaded_size": 0
				},
				"remote": {
					"@type": "remoteFile",
					"id": "AQADAQADqacxGyyuUSwACL8DCzAABHLmUqWpsqcqdgoDAAEC",
					"is_uploading_active": false,
					"is_uploading_completed": true,
					"uploaded_size": 0
				}
			}
		},
		"outgoing_link": {
			"@type": "linkStateNone"
		},
		"incoming_link": {
			"@type": "linkStateNone"
		},
		"is_verified": false,
		"is_support": false,
		"restriction_reason": "",
		"have_access": true,
		"type": {
			"@type": "userTypeRegular"
		},
		"language_code": ""
	}
}
  "#;

  let object: Option<Box<rtdlib::types::Object>> = rtdlib::types::Object::from_json(json);
  println!("{:?}", object);
}

