macro_rules! gen_merkle_tree_large {
      () => {{
          vec![
              (hex!("0000000000000000000000000000000000000000000000000000000000000011")).into(),
              (hex!("0000000000000000000000000000000000000000000000000000000000000022")).into(),
              (hex!("0000000000000000000000000000000000000000000000000000000000000033")).into(),
              (hex!("0000000000000000000000000000000000000000000000000000000000000044")).into(),
              (hex!("0000000000000000000000000000000000000000000000000000000000000055")).into(),
              (hex!("0000000000000000000000000000000000000000000000000000000000000066")).into(),
              (hex!("0000000000000000000000000000000000000000000000000000000000000077")).into(),
              (hex!("0000000000000000000000000000000000000000000000000000000000000088")).into(),
          ]
      }};
  }

  #[test]
  fn proof_tree_large() {
      let input_data: Vec<H256> = gen_merkle_tree_large!();
      let merkle_tree = MerkleTree::new(&input_data);
      let proof = merkle_tree.proof(5);

      // We accept the proof in either the top-down or bottom-up order; you should stick to either of them.
      let expected_proof_bottom_up: Vec<H256> = vec![
          (hex!("c8c37c89fcc6ee7f5e8237d2b7ed8c17640c154f8d7751c774719b2b82040c76")).into(),
          (hex!("bada70a695501195fb5ad950a5a41c02c0f9c449a918937267710a0425151b77")).into(),
          (hex!("1e28fb71415f259bd4b0b3b98d67a1240b4f3bed5923aa222c5fdbd97c8fb002")).into(),
      ];
      let expected_proof_top_down: Vec<H256> = vec![
          (hex!("1e28fb71415f259bd4b0b3b98d67a1240b4f3bed5923aa222c5fdbd97c8fb002")).into(),  
          (hex!("bada70a695501195fb5ad950a5a41c02c0f9c449a918937267710a0425151b77")).into(),
          (hex!("c8c37c89fcc6ee7f5e8237d2b7ed8c17640c154f8d7751c774719b2b82040c76")).into(),
      ];
      assert!(proof == expected_proof_bottom_up || proof == expected_proof_top_down);
  }
