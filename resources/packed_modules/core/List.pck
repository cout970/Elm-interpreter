{
  "header": {
    "name": "List",
    "exposing": {
      "Just": [
        {
          "Definition": "singleton"
        },
        {
          "Definition": "repeat"
        },
        {
          "Definition": "range"
        },
        {
          "BinaryOperator": "::"
        },
        {
          "Definition": "map"
        },
        {
          "Definition": "indexedMap"
        },
        {
          "Definition": "foldl"
        },
        {
          "Definition": "foldr"
        },
        {
          "Definition": "filter"
        },
        {
          "Definition": "filterMap"
        },
        {
          "Definition": "length"
        },
        {
          "Definition": "reverse"
        },
        {
          "Definition": "member"
        },
        {
          "Definition": "all"
        },
        {
          "Definition": "any"
        },
        {
          "Definition": "maximum"
        },
        {
          "Definition": "minimum"
        },
        {
          "Definition": "sum"
        },
        {
          "Definition": "product"
        },
        {
          "Definition": "append"
        },
        {
          "Definition": "concat"
        },
        {
          "Definition": "concatMap"
        },
        {
          "Definition": "intersperse"
        },
        {
          "Definition": "map2"
        },
        {
          "Definition": "map3"
        },
        {
          "Definition": "map4"
        },
        {
          "Definition": "map5"
        },
        {
          "Definition": "sort"
        },
        {
          "Definition": "sortBy"
        },
        {
          "Definition": "sortWith"
        },
        {
          "Definition": "isEmpty"
        },
        {
          "Definition": "head"
        },
        {
          "Definition": "tail"
        },
        {
          "Definition": "take"
        },
        {
          "Definition": "drop"
        },
        {
          "Definition": "partition"
        },
        {
          "Definition": "unzip"
        }
      ]
    }
  },
  "imports": [
    {
      "path": [
        "Basics"
      ],
      "alias": null,
      "exposing": "All"
    },
    {
      "path": [
        "Elm",
        "Kernel",
        "List"
      ],
      "alias": null,
      "exposing": null
    },
    {
      "path": [
        "Maybe"
      ],
      "alias": null,
      "exposing": {
        "Just": [
          {
            "Adt": [
              "Maybe",
              "All"
            ]
          }
        ]
      }
    }
  ],
  "statements": [
    {
      "Infix": [
        "right",
        5,
        "::",
        "cons"
      ]
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Var": "a"
            },
            {
              "Tag": [
                "List",
                [
                  {
                    "Var": "a"
                  }
                ]
              ]
            }
          ]
        },
        "name": "singleton",
        "patterns": [
          {
            "Var": "value"
          }
        ],
        "expr": {
          "List": [
            [
              1170,
              1177
            ],
            [
              {
                "Ref": [
                  [
                    1171,
                    1176
                  ],
                  "value"
                ]
              }
            ]
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Int",
                []
              ]
            },
            {
              "Fun": [
                {
                  "Var": "a"
                },
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                }
              ]
            }
          ]
        },
        "name": "repeat",
        "patterns": [
          {
            "Var": "n"
          },
          {
            "Var": "value"
          }
        ],
        "expr": {
          "Application": [
            [
              1319,
              1340
            ],
            {
              "Application": [
                [
                  1319,
                  1340
                ],
                {
                  "Application": [
                    [
                      1319,
                      1340
                    ],
                    {
                      "Ref": [
                        [
                          1319,
                          1329
                        ],
                        "repeatHelp"
                      ]
                    },
                    {
                      "List": [
                        [
                          1330,
                          1333
                        ],
                        []
                      ]
                    }
                  ]
                },
                {
                  "Ref": [
                    [
                      1333,
                      1334
                    ],
                    "n"
                  ]
                }
              ]
            },
            {
              "Ref": [
                [
                  1335,
                  1340
                ],
                "value"
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "List",
                [
                  {
                    "Var": "a"
                  }
                ]
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Int",
                    []
                  ]
                },
                {
                  "Fun": [
                    {
                      "Var": "a"
                    },
                    {
                      "Tag": [
                        "List",
                        [
                          {
                            "Var": "a"
                          }
                        ]
                      ]
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "repeatHelp",
        "patterns": [
          {
            "Var": "result"
          },
          {
            "Var": "n"
          },
          {
            "Var": "value"
          }
        ],
        "expr": {
          "If": [
            [
              1415,
              1497
            ],
            {
              "OpChain": [
                [
                  1418,
                  1424
                ],
                [
                  {
                    "Ref": [
                      [
                        1418,
                        1419
                      ],
                      "n"
                    ]
                  },
                  {
                    "Literal": [
                      [
                        1423,
                        1424
                      ],
                      {
                        "Int": 0
                      }
                    ]
                  }
                ],
                [
                  "<="
                ]
              ]
            },
            {
              "Ref": [
                [
                  1434,
                  1440
                ],
                "result"
              ]
            },
            {
              "Application": [
                [
                  1453,
                  1497
                ],
                {
                  "Application": [
                    [
                      1453,
                      1497
                    ],
                    {
                      "Application": [
                        [
                          1453,
                          1497
                        ],
                        {
                          "Ref": [
                            [
                              1453,
                              1463
                            ],
                            "repeatHelp"
                          ]
                        },
                        {
                          "Application": [
                            [
                              1465,
                              1482
                            ],
                            {
                              "Application": [
                                [
                                  1465,
                                  1482
                                ],
                                {
                                  "Ref": [
                                    [
                                      1465,
                                      1469
                                    ],
                                    "cons"
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      1470,
                                      1475
                                    ],
                                    "value"
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  1476,
                                  1482
                                ],
                                "result"
                              ]
                            }
                          ]
                        }
                      ]
                    },
                    {
                      "OpChain": [
                        [
                          1485,
                          1490
                        ],
                        [
                          {
                            "Ref": [
                              [
                                1485,
                                1486
                              ],
                              "n"
                            ]
                          },
                          {
                            "Literal": [
                              [
                                1489,
                                1490
                              ],
                              {
                                "Int": 1
                              }
                            ]
                          }
                        ],
                        [
                          "-"
                        ]
                      ]
                    }
                  ]
                },
                {
                  "Ref": [
                    [
                      1492,
                      1497
                    ],
                    "value"
                  ]
                }
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Int",
                []
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Int",
                    []
                  ]
                },
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Tag": [
                          "Int",
                          []
                        ]
                      }
                    ]
                  ]
                }
              ]
            }
          ]
        },
        "name": "range",
        "patterns": [
          {
            "Var": "lo"
          },
          {
            "Var": "hi"
          }
        ],
        "expr": {
          "Application": [
            [
              1752,
              1770
            ],
            {
              "Application": [
                [
                  1752,
                  1770
                ],
                {
                  "Application": [
                    [
                      1752,
                      1770
                    ],
                    {
                      "Ref": [
                        [
                          1752,
                          1761
                        ],
                        "rangeHelp"
                      ]
                    },
                    {
                      "Ref": [
                        [
                          1762,
                          1764
                        ],
                        "lo"
                      ]
                    }
                  ]
                },
                {
                  "Ref": [
                    [
                      1765,
                      1767
                    ],
                    "hi"
                  ]
                }
              ]
            },
            {
              "List": [
                [
                  1768,
                  1770
                ],
                []
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Int",
                []
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Int",
                    []
                  ]
                },
                {
                  "Fun": [
                    {
                      "Tag": [
                        "List",
                        [
                          {
                            "Tag": [
                              "Int",
                              []
                            ]
                          }
                        ]
                      ]
                    },
                    {
                      "Tag": [
                        "List",
                        [
                          {
                            "Tag": [
                              "Int",
                              []
                            ]
                          }
                        ]
                      ]
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "rangeHelp",
        "patterns": [
          {
            "Var": "lo"
          },
          {
            "Var": "hi"
          },
          {
            "Var": "list"
          }
        ],
        "expr": {
          "If": [
            [
              1845,
              1919
            ],
            {
              "OpChain": [
                [
                  1848,
                  1856
                ],
                [
                  {
                    "Ref": [
                      [
                        1848,
                        1850
                      ],
                      "lo"
                    ]
                  },
                  {
                    "Ref": [
                      [
                        1854,
                        1856
                      ],
                      "hi"
                    ]
                  }
                ],
                [
                  "<="
                ]
              ]
            },
            {
              "Application": [
                [
                  1866,
                  1902
                ],
                {
                  "Application": [
                    [
                      1866,
                      1902
                    ],
                    {
                      "Application": [
                        [
                          1866,
                          1902
                        ],
                        {
                          "Ref": [
                            [
                              1866,
                              1875
                            ],
                            "rangeHelp"
                          ]
                        },
                        {
                          "Ref": [
                            [
                              1876,
                              1878
                            ],
                            "lo"
                          ]
                        }
                      ]
                    },
                    {
                      "OpChain": [
                        [
                          1880,
                          1886
                        ],
                        [
                          {
                            "Ref": [
                              [
                                1880,
                                1882
                              ],
                              "hi"
                            ]
                          },
                          {
                            "Literal": [
                              [
                                1885,
                                1886
                              ],
                              {
                                "Int": 1
                              }
                            ]
                          }
                        ],
                        [
                          "-"
                        ]
                      ]
                    }
                  ]
                },
                {
                  "Application": [
                    [
                      1889,
                      1901
                    ],
                    {
                      "Application": [
                        [
                          1889,
                          1901
                        ],
                        {
                          "Ref": [
                            [
                              1889,
                              1893
                            ],
                            "cons"
                          ]
                        },
                        {
                          "Ref": [
                            [
                              1894,
                              1896
                            ],
                            "hi"
                          ]
                        }
                      ]
                    },
                    {
                      "Ref": [
                        [
                          1897,
                          1901
                        ],
                        "list"
                      ]
                    }
                  ]
                }
              ]
            },
            {
              "Ref": [
                [
                  1915,
                  1919
                ],
                "list"
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Var": "a"
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                }
              ]
            }
          ]
        },
        "name": "cons",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              2172,
              2192
            ],
            [
              "Elm",
              "Kernel",
              "List"
            ],
            "cons"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Fun": [
                {
                  "Var": "a"
                },
                {
                  "Var": "b"
                }
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "b"
                      }
                    ]
                  ]
                }
              ]
            }
          ]
        },
        "name": "map",
        "patterns": [
          {
            "Var": "f"
          },
          {
            "Var": "xs"
          }
        ],
        "expr": {
          "Application": [
            [
              2468,
              2506
            ],
            {
              "Application": [
                [
                  2468,
                  2506
                ],
                {
                  "Application": [
                    [
                      2468,
                      2506
                    ],
                    {
                      "Ref": [
                        [
                          2468,
                          2473
                        ],
                        "foldr"
                      ]
                    },
                    {
                      "Lambda": [
                        [
                          2475,
                          2499
                        ],
                        [
                          {
                            "Var": "x"
                          },
                          {
                            "Var": "acc"
                          }
                        ],
                        {
                          "Application": [
                            [
                              2485,
                              2499
                            ],
                            {
                              "Application": [
                                [
                                  2485,
                                  2499
                                ],
                                {
                                  "Ref": [
                                    [
                                      2485,
                                      2489
                                    ],
                                    "cons"
                                  ]
                                },
                                {
                                  "Application": [
                                    [
                                      2491,
                                      2494
                                    ],
                                    {
                                      "Ref": [
                                        [
                                          2491,
                                          2492
                                        ],
                                        "f"
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          2493,
                                          2494
                                        ],
                                        "x"
                                      ]
                                    }
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  2496,
                                  2499
                                ],
                                "acc"
                              ]
                            }
                          ]
                        }
                      ]
                    }
                  ]
                },
                {
                  "List": [
                    [
                      2501,
                      2504
                    ],
                    []
                  ]
                }
              ]
            },
            {
              "Ref": [
                [
                  2504,
                  2506
                ],
                "xs"
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Fun": [
                {
                  "Tag": [
                    "Int",
                    []
                  ]
                },
                {
                  "Fun": [
                    {
                      "Var": "a"
                    },
                    {
                      "Var": "b"
                    }
                  ]
                }
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "b"
                      }
                    ]
                  ]
                }
              ]
            }
          ]
        },
        "name": "indexedMap",
        "patterns": [
          {
            "Var": "f"
          },
          {
            "Var": "xs"
          }
        ],
        "expr": {
          "Application": [
            [
              2767,
              2802
            ],
            {
              "Application": [
                [
                  2767,
                  2802
                ],
                {
                  "Application": [
                    [
                      2767,
                      2802
                    ],
                    {
                      "Ref": [
                        [
                          2767,
                          2771
                        ],
                        "map2"
                      ]
                    },
                    {
                      "Ref": [
                        [
                          2772,
                          2773
                        ],
                        "f"
                      ]
                    }
                  ]
                },
                {
                  "Application": [
                    [
                      2775,
                      2798
                    ],
                    {
                      "Application": [
                        [
                          2775,
                          2798
                        ],
                        {
                          "Ref": [
                            [
                              2775,
                              2780
                            ],
                            "range"
                          ]
                        },
                        {
                          "Literal": [
                            [
                              2781,
                              2782
                            ],
                            {
                              "Int": 0
                            }
                          ]
                        }
                      ]
                    },
                    {
                      "OpChain": [
                        [
                          2784,
                          2797
                        ],
                        [
                          {
                            "Application": [
                              [
                                2784,
                                2793
                              ],
                              {
                                "Ref": [
                                  [
                                    2784,
                                    2790
                                  ],
                                  "length"
                                ]
                              },
                              {
                                "Ref": [
                                  [
                                    2791,
                                    2793
                                  ],
                                  "xs"
                                ]
                              }
                            ]
                          },
                          {
                            "Literal": [
                              [
                                2796,
                                2797
                              ],
                              {
                                "Int": 1
                              }
                            ]
                          }
                        ],
                        [
                          "-"
                        ]
                      ]
                    }
                  ]
                }
              ]
            },
            {
              "Ref": [
                [
                  2800,
                  2802
                ],
                "xs"
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Fun": [
                {
                  "Var": "a"
                },
                {
                  "Fun": [
                    {
                      "Var": "b"
                    },
                    {
                      "Var": "b"
                    }
                  ]
                }
              ]
            },
            {
              "Fun": [
                {
                  "Var": "b"
                },
                {
                  "Fun": [
                    {
                      "Tag": [
                        "List",
                        [
                          {
                            "Var": "a"
                          }
                        ]
                      ]
                    },
                    {
                      "Var": "b"
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "foldl",
        "patterns": [
          {
            "Var": "func"
          },
          {
            "Var": "acc"
          },
          {
            "Var": "list"
          }
        ],
        "expr": {
          "Case": [
            [
              3082,
              3163
            ],
            {
              "Ref": [
                [
                  3087,
                  3091
                ],
                "list"
              ]
            },
            [
              [
                {
                  "List": []
                },
                {
                  "Ref": [
                    [
                      3111,
                      3114
                    ],
                    "acc"
                  ]
                }
              ],
              [
                {
                  "BinaryOp": [
                    "::",
                    {
                      "Var": "x"
                    },
                    {
                      "Var": "xs"
                    }
                  ]
                },
                {
                  "Application": [
                    [
                      3137,
                      3163
                    ],
                    {
                      "Application": [
                        [
                          3137,
                          3163
                        ],
                        {
                          "Application": [
                            [
                              3137,
                              3163
                            ],
                            {
                              "Ref": [
                                [
                                  3137,
                                  3142
                                ],
                                "foldl"
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  3143,
                                  3147
                                ],
                                "func"
                              ]
                            }
                          ]
                        },
                        {
                          "Application": [
                            [
                              3149,
                              3159
                            ],
                            {
                              "Application": [
                                [
                                  3149,
                                  3159
                                ],
                                {
                                  "Ref": [
                                    [
                                      3149,
                                      3153
                                    ],
                                    "func"
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      3154,
                                      3155
                                    ],
                                    "x"
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  3156,
                                  3159
                                ],
                                "acc"
                              ]
                            }
                          ]
                        }
                      ]
                    },
                    {
                      "Ref": [
                        [
                          3161,
                          3163
                        ],
                        "xs"
                      ]
                    }
                  ]
                }
              ]
            ]
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Fun": [
                {
                  "Var": "a"
                },
                {
                  "Fun": [
                    {
                      "Var": "b"
                    },
                    {
                      "Var": "b"
                    }
                  ]
                }
              ]
            },
            {
              "Fun": [
                {
                  "Var": "b"
                },
                {
                  "Fun": [
                    {
                      "Tag": [
                        "List",
                        [
                          {
                            "Var": "a"
                          }
                        ]
                      ]
                    },
                    {
                      "Var": "b"
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "foldr",
        "patterns": [
          {
            "Var": "fn"
          },
          {
            "Var": "acc"
          },
          {
            "Var": "ls"
          }
        ],
        "expr": {
          "Application": [
            [
              3442,
              3465
            ],
            {
              "Application": [
                [
                  3442,
                  3465
                ],
                {
                  "Application": [
                    [
                      3442,
                      3465
                    ],
                    {
                      "Application": [
                        [
                          3442,
                          3465
                        ],
                        {
                          "Ref": [
                            [
                              3442,
                              3453
                            ],
                            "foldrHelper"
                          ]
                        },
                        {
                          "Ref": [
                            [
                              3454,
                              3456
                            ],
                            "fn"
                          ]
                        }
                      ]
                    },
                    {
                      "Ref": [
                        [
                          3457,
                          3460
                        ],
                        "acc"
                      ]
                    }
                  ]
                },
                {
                  "Literal": [
                    [
                      3461,
                      3462
                    ],
                    {
                      "Int": 0
                    }
                  ]
                }
              ]
            },
            {
              "Ref": [
                [
                  3463,
                  3465
                ],
                "ls"
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Fun": [
                {
                  "Var": "a"
                },
                {
                  "Fun": [
                    {
                      "Var": "b"
                    },
                    {
                      "Var": "b"
                    }
                  ]
                }
              ]
            },
            {
              "Fun": [
                {
                  "Var": "b"
                },
                {
                  "Fun": [
                    {
                      "Tag": [
                        "Int",
                        []
                      ]
                    },
                    {
                      "Fun": [
                        {
                          "Tag": [
                            "List",
                            [
                              {
                                "Var": "a"
                              }
                            ]
                          ]
                        },
                        {
                          "Var": "b"
                        }
                      ]
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "foldrHelper",
        "patterns": [
          {
            "Var": "fn"
          },
          {
            "Var": "acc"
          },
          {
            "Var": "ctr"
          },
          {
            "Var": "ls"
          }
        ],
        "expr": {
          "Case": [
            [
              3555,
              4498
            ],
            {
              "Ref": [
                [
                  3560,
                  3562
                ],
                "ls"
              ]
            },
            [
              [
                {
                  "List": []
                },
                {
                  "Ref": [
                    [
                      3592,
                      3595
                    ],
                    "acc"
                  ]
                }
              ],
              [
                {
                  "BinaryOp": [
                    "::",
                    {
                      "Var": "a"
                    },
                    {
                      "Var": "r1"
                    }
                  ]
                },
                {
                  "Case": [
                    [
                      3628,
                      4498
                    ],
                    {
                      "Ref": [
                        [
                          3633,
                          3635
                        ],
                        "r1"
                      ]
                    },
                    [
                      [
                        {
                          "List": []
                        },
                        {
                          "Application": [
                            [
                              3681,
                              3689
                            ],
                            {
                              "Application": [
                                [
                                  3681,
                                  3689
                                ],
                                {
                                  "Ref": [
                                    [
                                      3681,
                                      3683
                                    ],
                                    "fn"
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      3684,
                                      3685
                                    ],
                                    "a"
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  3686,
                                  3689
                                ],
                                "acc"
                              ]
                            }
                          ]
                        }
                      ],
                      [
                        {
                          "BinaryOp": [
                            "::",
                            {
                              "Var": "b"
                            },
                            {
                              "Var": "r2"
                            }
                          ]
                        },
                        {
                          "Case": [
                            [
                              3738,
                              4498
                            ],
                            {
                              "Ref": [
                                [
                                  3743,
                                  3745
                                ],
                                "r2"
                              ]
                            },
                            [
                              [
                                {
                                  "List": []
                                },
                                {
                                  "Application": [
                                    [
                                      3807,
                                      3822
                                    ],
                                    {
                                      "Application": [
                                        [
                                          3807,
                                          3822
                                        ],
                                        {
                                          "Ref": [
                                            [
                                              3807,
                                              3809
                                            ],
                                            "fn"
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              3810,
                                              3811
                                            ],
                                            "a"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Application": [
                                        [
                                          3813,
                                          3821
                                        ],
                                        {
                                          "Application": [
                                            [
                                              3813,
                                              3821
                                            ],
                                            {
                                              "Ref": [
                                                [
                                                  3813,
                                                  3815
                                                ],
                                                "fn"
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  3816,
                                                  3817
                                                ],
                                                "b"
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              3818,
                                              3821
                                            ],
                                            "acc"
                                          ]
                                        }
                                      ]
                                    }
                                  ]
                                }
                              ],
                              [
                                {
                                  "BinaryOp": [
                                    "::",
                                    {
                                      "Var": "c"
                                    },
                                    {
                                      "Var": "r3"
                                    }
                                  ]
                                },
                                {
                                  "Case": [
                                    [
                                      3887,
                                      4498
                                    ],
                                    {
                                      "Ref": [
                                        [
                                          3892,
                                          3894
                                        ],
                                        "r3"
                                      ]
                                    },
                                    [
                                      [
                                        {
                                          "List": []
                                        },
                                        {
                                          "Application": [
                                            [
                                              3972,
                                              3994
                                            ],
                                            {
                                              "Application": [
                                                [
                                                  3972,
                                                  3994
                                                ],
                                                {
                                                  "Ref": [
                                                    [
                                                      3972,
                                                      3974
                                                    ],
                                                    "fn"
                                                  ]
                                                },
                                                {
                                                  "Ref": [
                                                    [
                                                      3975,
                                                      3976
                                                    ],
                                                    "a"
                                                  ]
                                                }
                                              ]
                                            },
                                            {
                                              "Application": [
                                                [
                                                  3978,
                                                  3993
                                                ],
                                                {
                                                  "Application": [
                                                    [
                                                      3978,
                                                      3993
                                                    ],
                                                    {
                                                      "Ref": [
                                                        [
                                                          3978,
                                                          3980
                                                        ],
                                                        "fn"
                                                      ]
                                                    },
                                                    {
                                                      "Ref": [
                                                        [
                                                          3981,
                                                          3982
                                                        ],
                                                        "b"
                                                      ]
                                                    }
                                                  ]
                                                },
                                                {
                                                  "Application": [
                                                    [
                                                      3984,
                                                      3992
                                                    ],
                                                    {
                                                      "Application": [
                                                        [
                                                          3984,
                                                          3992
                                                        ],
                                                        {
                                                          "Ref": [
                                                            [
                                                              3984,
                                                              3986
                                                            ],
                                                            "fn"
                                                          ]
                                                        },
                                                        {
                                                          "Ref": [
                                                            [
                                                              3987,
                                                              3988
                                                            ],
                                                            "c"
                                                          ]
                                                        }
                                                      ]
                                                    },
                                                    {
                                                      "Ref": [
                                                        [
                                                          3989,
                                                          3992
                                                        ],
                                                        "acc"
                                                      ]
                                                    }
                                                  ]
                                                }
                                              ]
                                            }
                                          ]
                                        }
                                      ],
                                      [
                                        {
                                          "BinaryOp": [
                                            "::",
                                            {
                                              "Var": "d"
                                            },
                                            {
                                              "Var": "r4"
                                            }
                                          ]
                                        },
                                        {
                                          "Let": [
                                            [
                                              4075,
                                              4498
                                            ],
                                            [
                                              {
                                                "Def": {
                                                  "header": null,
                                                  "name": "res",
                                                  "patterns": [],
                                                  "expr": {
                                                    "If": [
                                                      [
                                                        4169,
                                                        4426
                                                      ],
                                                      {
                                                        "OpChain": [
                                                          [
                                                            4172,
                                                            4181
                                                          ],
                                                          [
                                                            {
                                                              "Ref": [
                                                                [
                                                                  4172,
                                                                  4175
                                                                ],
                                                                "ctr"
                                                              ]
                                                            },
                                                            {
                                                              "Literal": [
                                                                [
                                                                  4178,
                                                                  4181
                                                                ],
                                                                {
                                                                  "Int": 500
                                                                }
                                                              ]
                                                            }
                                                          ],
                                                          [
                                                            ">"
                                                          ]
                                                        ]
                                                      },
                                                      {
                                                        "Application": [
                                                          [
                                                            4235,
                                                            4260
                                                          ],
                                                          {
                                                            "Application": [
                                                              [
                                                                4235,
                                                                4260
                                                              ],
                                                              {
                                                                "Application": [
                                                                  [
                                                                    4235,
                                                                    4260
                                                                  ],
                                                                  {
                                                                    "Ref": [
                                                                      [
                                                                        4235,
                                                                        4240
                                                                      ],
                                                                      "foldl"
                                                                    ]
                                                                  },
                                                                  {
                                                                    "Ref": [
                                                                      [
                                                                        4241,
                                                                        4243
                                                                      ],
                                                                      "fn"
                                                                    ]
                                                                  }
                                                                ]
                                                              },
                                                              {
                                                                "Ref": [
                                                                  [
                                                                    4244,
                                                                    4247
                                                                  ],
                                                                  "acc"
                                                                ]
                                                              }
                                                            ]
                                                          },
                                                          {
                                                            "Application": [
                                                              [
                                                                4249,
                                                                4259
                                                              ],
                                                              {
                                                                "Ref": [
                                                                  [
                                                                    4249,
                                                                    4256
                                                                  ],
                                                                  "reverse"
                                                                ]
                                                              },
                                                              {
                                                                "Ref": [
                                                                  [
                                                                    4257,
                                                                    4259
                                                                  ],
                                                                  "r4"
                                                                ]
                                                              }
                                                            ]
                                                          }
                                                        ]
                                                      },
                                                      {
                                                        "Application": [
                                                          [
                                                            4358,
                                                            4389
                                                          ],
                                                          {
                                                            "Application": [
                                                              [
                                                                4358,
                                                                4389
                                                              ],
                                                              {
                                                                "Application": [
                                                                  [
                                                                    4358,
                                                                    4389
                                                                  ],
                                                                  {
                                                                    "Application": [
                                                                      [
                                                                        4358,
                                                                        4389
                                                                      ],
                                                                      {
                                                                        "Ref": [
                                                                          [
                                                                            4358,
                                                                            4369
                                                                          ],
                                                                          "foldrHelper"
                                                                        ]
                                                                      },
                                                                      {
                                                                        "Ref": [
                                                                          [
                                                                            4370,
                                                                            4372
                                                                          ],
                                                                          "fn"
                                                                        ]
                                                                      }
                                                                    ]
                                                                  },
                                                                  {
                                                                    "Ref": [
                                                                      [
                                                                        4373,
                                                                        4376
                                                                      ],
                                                                      "acc"
                                                                    ]
                                                                  }
                                                                ]
                                                              },
                                                              {
                                                                "OpChain": [
                                                                  [
                                                                    4378,
                                                                    4385
                                                                  ],
                                                                  [
                                                                    {
                                                                      "Ref": [
                                                                        [
                                                                          4378,
                                                                          4381
                                                                        ],
                                                                        "ctr"
                                                                      ]
                                                                    },
                                                                    {
                                                                      "Literal": [
                                                                        [
                                                                          4384,
                                                                          4385
                                                                        ],
                                                                        {
                                                                          "Int": 1
                                                                        }
                                                                      ]
                                                                    }
                                                                  ],
                                                                  [
                                                                    "+"
                                                                  ]
                                                                ]
                                                              }
                                                            ]
                                                          },
                                                          {
                                                            "Ref": [
                                                              [
                                                                4387,
                                                                4389
                                                              ],
                                                              "r4"
                                                            ]
                                                          }
                                                        ]
                                                      }
                                                    ]
                                                  }
                                                }
                                              }
                                            ],
                                            {
                                              "Application": [
                                                [
                                                  4469,
                                                  4498
                                                ],
                                                {
                                                  "Application": [
                                                    [
                                                      4469,
                                                      4498
                                                    ],
                                                    {
                                                      "Ref": [
                                                        [
                                                          4469,
                                                          4471
                                                        ],
                                                        "fn"
                                                      ]
                                                    },
                                                    {
                                                      "Ref": [
                                                        [
                                                          4472,
                                                          4473
                                                        ],
                                                        "a"
                                                      ]
                                                    }
                                                  ]
                                                },
                                                {
                                                  "Application": [
                                                    [
                                                      4475,
                                                      4497
                                                    ],
                                                    {
                                                      "Application": [
                                                        [
                                                          4475,
                                                          4497
                                                        ],
                                                        {
                                                          "Ref": [
                                                            [
                                                              4475,
                                                              4477
                                                            ],
                                                            "fn"
                                                          ]
                                                        },
                                                        {
                                                          "Ref": [
                                                            [
                                                              4478,
                                                              4479
                                                            ],
                                                            "b"
                                                          ]
                                                        }
                                                      ]
                                                    },
                                                    {
                                                      "Application": [
                                                        [
                                                          4481,
                                                          4496
                                                        ],
                                                        {
                                                          "Application": [
                                                            [
                                                              4481,
                                                              4496
                                                            ],
                                                            {
                                                              "Ref": [
                                                                [
                                                                  4481,
                                                                  4483
                                                                ],
                                                                "fn"
                                                              ]
                                                            },
                                                            {
                                                              "Ref": [
                                                                [
                                                                  4484,
                                                                  4485
                                                                ],
                                                                "c"
                                                              ]
                                                            }
                                                          ]
                                                        },
                                                        {
                                                          "Application": [
                                                            [
                                                              4487,
                                                              4495
                                                            ],
                                                            {
                                                              "Application": [
                                                                [
                                                                  4487,
                                                                  4495
                                                                ],
                                                                {
                                                                  "Ref": [
                                                                    [
                                                                      4487,
                                                                      4489
                                                                    ],
                                                                    "fn"
                                                                  ]
                                                                },
                                                                {
                                                                  "Ref": [
                                                                    [
                                                                      4490,
                                                                      4491
                                                                    ],
                                                                    "d"
                                                                  ]
                                                                }
                                                              ]
                                                            },
                                                            {
                                                              "Ref": [
                                                                [
                                                                  4492,
                                                                  4495
                                                                ],
                                                                "res"
                                                              ]
                                                            }
                                                          ]
                                                        }
                                                      ]
                                                    }
                                                  ]
                                                }
                                              ]
                                            }
                                          ]
                                        }
                                      ]
                                    ]
                                  ]
                                }
                              ]
                            ]
                          ]
                        }
                      ]
                    ]
                  ]
                }
              ]
            ]
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Fun": [
                {
                  "Var": "a"
                },
                {
                  "Tag": [
                    "Bool",
                    []
                  ]
                }
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                }
              ]
            }
          ]
        },
        "name": "filter",
        "patterns": [
          {
            "Var": "isGood"
          },
          {
            "Var": "list"
          }
        ],
        "expr": {
          "Application": [
            [
              4653,
              4712
            ],
            {
              "Application": [
                [
                  4653,
                  4712
                ],
                {
                  "Application": [
                    [
                      4653,
                      4712
                    ],
                    {
                      "Ref": [
                        [
                          4653,
                          4658
                        ],
                        "foldr"
                      ]
                    },
                    {
                      "Lambda": [
                        [
                          4660,
                          4703
                        ],
                        [
                          {
                            "Var": "x"
                          },
                          {
                            "Var": "xs"
                          }
                        ],
                        {
                          "If": [
                            [
                              4669,
                              4703
                            ],
                            {
                              "Application": [
                                [
                                  4672,
                                  4680
                                ],
                                {
                                  "Ref": [
                                    [
                                      4672,
                                      4678
                                    ],
                                    "isGood"
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      4679,
                                      4680
                                    ],
                                    "x"
                                  ]
                                }
                              ]
                            },
                            {
                              "Application": [
                                [
                                  4686,
                                  4695
                                ],
                                {
                                  "Application": [
                                    [
                                      4686,
                                      4695
                                    ],
                                    {
                                      "Ref": [
                                        [
                                          4686,
                                          4690
                                        ],
                                        "cons"
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          4691,
                                          4692
                                        ],
                                        "x"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      4693,
                                      4695
                                    ],
                                    "xs"
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  4701,
                                  4703
                                ],
                                "xs"
                              ]
                            }
                          ]
                        }
                      ]
                    }
                  ]
                },
                {
                  "List": [
                    [
                      4705,
                      4708
                    ],
                    []
                  ]
                }
              ]
            },
            {
              "Ref": [
                [
                  4708,
                  4712
                ],
                "list"
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Fun": [
                {
                  "Var": "a"
                },
                {
                  "Tag": [
                    "Maybe",
                    [
                      {
                        "Var": "b"
                      }
                    ]
                  ]
                }
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "b"
                      }
                    ]
                  ]
                }
              ]
            }
          ]
        },
        "name": "filterMap",
        "patterns": [
          {
            "Var": "f"
          },
          {
            "Var": "xs"
          }
        ],
        "expr": {
          "Application": [
            [
              5054,
              5079
            ],
            {
              "Application": [
                [
                  5054,
                  5079
                ],
                {
                  "Application": [
                    [
                      5054,
                      5079
                    ],
                    {
                      "Ref": [
                        [
                          5054,
                          5059
                        ],
                        "foldr"
                      ]
                    },
                    {
                      "Application": [
                        [
                          5061,
                          5072
                        ],
                        {
                          "Ref": [
                            [
                              5061,
                              5070
                            ],
                            "maybeCons"
                          ]
                        },
                        {
                          "Ref": [
                            [
                              5071,
                              5072
                            ],
                            "f"
                          ]
                        }
                      ]
                    }
                  ]
                },
                {
                  "List": [
                    [
                      5074,
                      5077
                    ],
                    []
                  ]
                }
              ]
            },
            {
              "Ref": [
                [
                  5077,
                  5079
                ],
                "xs"
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Fun": [
                {
                  "Var": "a"
                },
                {
                  "Tag": [
                    "Maybe",
                    [
                      {
                        "Var": "b"
                      }
                    ]
                  ]
                }
              ]
            },
            {
              "Fun": [
                {
                  "Var": "a"
                },
                {
                  "Fun": [
                    {
                      "Tag": [
                        "List",
                        [
                          {
                            "Var": "b"
                          }
                        ]
                      ]
                    },
                    {
                      "Tag": [
                        "List",
                        [
                          {
                            "Var": "b"
                          }
                        ]
                      ]
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "maybeCons",
        "patterns": [
          {
            "Var": "f"
          },
          {
            "Var": "mx"
          },
          {
            "Var": "xs"
          }
        ],
        "expr": {
          "Case": [
            [
              5156,
              5223
            ],
            {
              "Application": [
                [
                  5161,
                  5165
                ],
                {
                  "Ref": [
                    [
                      5161,
                      5162
                    ],
                    "f"
                  ]
                },
                {
                  "Ref": [
                    [
                      5163,
                      5165
                    ],
                    "mx"
                  ]
                }
              ]
            },
            [
              [
                {
                  "Adt": [
                    "Just",
                    [
                      {
                        "Var": "x"
                      }
                    ]
                  ]
                },
                {
                  "Application": [
                    [
                      5189,
                      5198
                    ],
                    {
                      "Application": [
                        [
                          5189,
                          5198
                        ],
                        {
                          "Ref": [
                            [
                              5189,
                              5193
                            ],
                            "cons"
                          ]
                        },
                        {
                          "Ref": [
                            [
                              5194,
                              5195
                            ],
                            "x"
                          ]
                        }
                      ]
                    },
                    {
                      "Ref": [
                        [
                          5196,
                          5198
                        ],
                        "xs"
                      ]
                    }
                  ]
                }
              ],
              [
                {
                  "Adt": [
                    "Nothing",
                    []
                  ]
                },
                {
                  "Ref": [
                    [
                      5221,
                      5223
                    ],
                    "xs"
                  ]
                }
              ]
            ]
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "List",
                [
                  {
                    "Var": "a"
                  }
                ]
              ]
            },
            {
              "Tag": [
                "Int",
                []
              ]
            }
          ]
        },
        "name": "length",
        "patterns": [
          {
            "Var": "xs"
          }
        ],
        "expr": {
          "Application": [
            [
              5342,
              5368
            ],
            {
              "Application": [
                [
                  5342,
                  5368
                ],
                {
                  "Application": [
                    [
                      5342,
                      5368
                    ],
                    {
                      "Ref": [
                        [
                          5342,
                          5347
                        ],
                        "foldl"
                      ]
                    },
                    {
                      "Lambda": [
                        [
                          5349,
                          5362
                        ],
                        [
                          "Wildcard",
                          {
                            "Var": "i"
                          }
                        ],
                        {
                          "OpChain": [
                            [
                              5357,
                              5362
                            ],
                            [
                              {
                                "Ref": [
                                  [
                                    5357,
                                    5358
                                  ],
                                  "i"
                                ]
                              },
                              {
                                "Literal": [
                                  [
                                    5361,
                                    5362
                                  ],
                                  {
                                    "Int": 1
                                  }
                                ]
                              }
                            ],
                            [
                              "+"
                            ]
                          ]
                        }
                      ]
                    }
                  ]
                },
                {
                  "Literal": [
                    [
                      5364,
                      5365
                    ],
                    {
                      "Int": 0
                    }
                  ]
                }
              ]
            },
            {
              "Ref": [
                [
                  5366,
                  5368
                ],
                "xs"
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "List",
                [
                  {
                    "Var": "a"
                  }
                ]
              ]
            },
            {
              "Tag": [
                "List",
                [
                  {
                    "Var": "a"
                  }
                ]
              ]
            }
          ]
        },
        "name": "reverse",
        "patterns": [
          {
            "Var": "list"
          }
        ],
        "expr": {
          "Application": [
            [
              5474,
              5492
            ],
            {
              "Application": [
                [
                  5474,
                  5492
                ],
                {
                  "Application": [
                    [
                      5474,
                      5492
                    ],
                    {
                      "Ref": [
                        [
                          5474,
                          5479
                        ],
                        "foldl"
                      ]
                    },
                    {
                      "Ref": [
                        [
                          5480,
                          5484
                        ],
                        "cons"
                      ]
                    }
                  ]
                },
                {
                  "List": [
                    [
                      5485,
                      5488
                    ],
                    []
                  ]
                }
              ]
            },
            {
              "Ref": [
                [
                  5488,
                  5492
                ],
                "list"
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Var": "a"
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "Bool",
                    []
                  ]
                }
              ]
            }
          ]
        },
        "name": "member",
        "patterns": [
          {
            "Var": "x"
          },
          {
            "Var": "xs"
          }
        ],
        "expr": {
          "Application": [
            [
              5655,
              5676
            ],
            {
              "Application": [
                [
                  5655,
                  5676
                ],
                {
                  "Ref": [
                    [
                      5655,
                      5658
                    ],
                    "any"
                  ]
                },
                {
                  "Lambda": [
                    [
                      5660,
                      5672
                    ],
                    [
                      {
                        "Var": "a"
                      }
                    ],
                    {
                      "OpChain": [
                        [
                          5666,
                          5672
                        ],
                        [
                          {
                            "Ref": [
                              [
                                5666,
                                5667
                              ],
                              "a"
                            ]
                          },
                          {
                            "Ref": [
                              [
                                5671,
                                5672
                              ],
                              "x"
                            ]
                          }
                        ],
                        [
                          "=="
                        ]
                      ]
                    }
                  ]
                }
              ]
            },
            {
              "Ref": [
                [
                  5674,
                  5676
                ],
                "xs"
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Fun": [
                {
                  "Var": "a"
                },
                {
                  "Tag": [
                    "Bool",
                    []
                  ]
                }
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "Bool",
                    []
                  ]
                }
              ]
            }
          ]
        },
        "name": "all",
        "patterns": [
          {
            "Var": "isOkay"
          },
          {
            "Var": "list"
          }
        ],
        "expr": {
          "Application": [
            [
              5873,
              5903
            ],
            {
              "Ref": [
                [
                  5873,
                  5876
                ],
                "not"
              ]
            },
            {
              "Application": [
                [
                  5878,
                  5902
                ],
                {
                  "Application": [
                    [
                      5878,
                      5902
                    ],
                    {
                      "Ref": [
                        [
                          5878,
                          5881
                        ],
                        "any"
                      ]
                    },
                    {
                      "OpChain": [
                        [
                          5883,
                          5896
                        ],
                        [
                          {
                            "Ref": [
                              [
                                5883,
                                5886
                              ],
                              "not"
                            ]
                          },
                          {
                            "Ref": [
                              [
                                5890,
                                5896
                              ],
                              "isOkay"
                            ]
                          }
                        ],
                        [
                          "<<"
                        ]
                      ]
                    }
                  ]
                },
                {
                  "Ref": [
                    [
                      5898,
                      5902
                    ],
                    "list"
                  ]
                }
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Fun": [
                {
                  "Var": "a"
                },
                {
                  "Tag": [
                    "Bool",
                    []
                  ]
                }
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "Bool",
                    []
                  ]
                }
              ]
            }
          ]
        },
        "name": "any",
        "patterns": [
          {
            "Var": "isOkay"
          },
          {
            "Var": "list"
          }
        ],
        "expr": {
          "Case": [
            [
              6101,
              6282
            ],
            {
              "Ref": [
                [
                  6106,
                  6110
                ],
                "list"
              ]
            },
            [
              [
                {
                  "List": []
                },
                {
                  "Ref": [
                    [
                      6130,
                      6135
                    ],
                    "False"
                  ]
                }
              ],
              [
                {
                  "BinaryOp": [
                    "::",
                    {
                      "Var": "x"
                    },
                    {
                      "Var": "xs"
                    }
                  ]
                },
                {
                  "If": [
                    [
                      6219,
                      6282
                    ],
                    {
                      "Application": [
                        [
                          6222,
                          6230
                        ],
                        {
                          "Ref": [
                            [
                              6222,
                              6228
                            ],
                            "isOkay"
                          ]
                        },
                        {
                          "Ref": [
                            [
                              6229,
                              6230
                            ],
                            "x"
                          ]
                        }
                      ]
                    },
                    {
                      "Ref": [
                        [
                          6244,
                          6256
                        ],
                        "True"
                      ]
                    },
                    {
                      "Application": [
                        [
                          6269,
                          6282
                        ],
                        {
                          "Application": [
                            [
                              6269,
                              6282
                            ],
                            {
                              "Ref": [
                                [
                                  6269,
                                  6272
                                ],
                                "any"
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  6273,
                                  6279
                                ],
                                "isOkay"
                              ]
                            }
                          ]
                        },
                        {
                          "Ref": [
                            [
                              6280,
                              6282
                            ],
                            "xs"
                          ]
                        }
                      ]
                    }
                  ]
                }
              ]
            ]
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "List",
                [
                  {
                    "Var": "comparable"
                  }
                ]
              ]
            },
            {
              "Tag": [
                "Maybe",
                [
                  {
                    "Var": "comparable"
                  }
                ]
              ]
            }
          ]
        },
        "name": "maximum",
        "patterns": [
          {
            "Var": "list"
          }
        ],
        "expr": {
          "Case": [
            [
              6463,
              6542
            ],
            {
              "Ref": [
                [
                  6468,
                  6472
                ],
                "list"
              ]
            },
            [
              [
                {
                  "BinaryOp": [
                    "::",
                    {
                      "Var": "x"
                    },
                    {
                      "Var": "xs"
                    }
                  ]
                },
                {
                  "Application": [
                    [
                      6497,
                      6518
                    ],
                    {
                      "Ref": [
                        [
                          6497,
                          6502
                        ],
                        "Just"
                      ]
                    },
                    {
                      "Application": [
                        [
                          6503,
                          6517
                        ],
                        {
                          "Application": [
                            [
                              6503,
                              6517
                            ],
                            {
                              "Application": [
                                [
                                  6503,
                                  6517
                                ],
                                {
                                  "Ref": [
                                    [
                                      6503,
                                      6508
                                    ],
                                    "foldl"
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      6509,
                                      6512
                                    ],
                                    "max"
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  6513,
                                  6514
                                ],
                                "x"
                              ]
                            }
                          ]
                        },
                        {
                          "Ref": [
                            [
                              6515,
                              6517
                            ],
                            "xs"
                          ]
                        }
                      ]
                    }
                  ]
                }
              ],
              [
                "Wildcard",
                {
                  "Ref": [
                    [
                      6535,
                      6542
                    ],
                    "Nothing"
                  ]
                }
              ]
            ]
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "List",
                [
                  {
                    "Var": "comparable"
                  }
                ]
              ]
            },
            {
              "Tag": [
                "Maybe",
                [
                  {
                    "Var": "comparable"
                  }
                ]
              ]
            }
          ]
        },
        "name": "minimum",
        "patterns": [
          {
            "Var": "list"
          }
        ],
        "expr": {
          "Case": [
            [
              6723,
              6802
            ],
            {
              "Ref": [
                [
                  6728,
                  6732
                ],
                "list"
              ]
            },
            [
              [
                {
                  "BinaryOp": [
                    "::",
                    {
                      "Var": "x"
                    },
                    {
                      "Var": "xs"
                    }
                  ]
                },
                {
                  "Application": [
                    [
                      6757,
                      6778
                    ],
                    {
                      "Ref": [
                        [
                          6757,
                          6762
                        ],
                        "Just"
                      ]
                    },
                    {
                      "Application": [
                        [
                          6763,
                          6777
                        ],
                        {
                          "Application": [
                            [
                              6763,
                              6777
                            ],
                            {
                              "Application": [
                                [
                                  6763,
                                  6777
                                ],
                                {
                                  "Ref": [
                                    [
                                      6763,
                                      6768
                                    ],
                                    "foldl"
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      6769,
                                      6772
                                    ],
                                    "min"
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  6773,
                                  6774
                                ],
                                "x"
                              ]
                            }
                          ]
                        },
                        {
                          "Ref": [
                            [
                              6775,
                              6777
                            ],
                            "xs"
                          ]
                        }
                      ]
                    }
                  ]
                }
              ],
              [
                "Wildcard",
                {
                  "Ref": [
                    [
                      6795,
                      6802
                    ],
                    "Nothing"
                  ]
                }
              ]
            ]
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "List",
                [
                  {
                    "Var": "number"
                  }
                ]
              ]
            },
            {
              "Var": "number"
            }
          ]
        },
        "name": "sum",
        "patterns": [
          {
            "Var": "numbers"
          }
        ],
        "expr": {
          "Application": [
            [
              6915,
              6934
            ],
            {
              "Application": [
                [
                  6915,
                  6934
                ],
                {
                  "Application": [
                    [
                      6915,
                      6934
                    ],
                    {
                      "Ref": [
                        [
                          6915,
                          6920
                        ],
                        "foldl"
                      ]
                    },
                    {
                      "Ref": [
                        [
                          6921,
                          6925
                        ],
                        "+"
                      ]
                    }
                  ]
                },
                {
                  "Literal": [
                    [
                      6925,
                      6926
                    ],
                    {
                      "Int": 0
                    }
                  ]
                }
              ]
            },
            {
              "Ref": [
                [
                  6927,
                  6934
                ],
                "numbers"
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "List",
                [
                  {
                    "Var": "number"
                  }
                ]
              ]
            },
            {
              "Var": "number"
            }
          ]
        },
        "name": "product",
        "patterns": [
          {
            "Var": "numbers"
          }
        ],
        "expr": {
          "Application": [
            [
              7063,
              7082
            ],
            {
              "Application": [
                [
                  7063,
                  7082
                ],
                {
                  "Application": [
                    [
                      7063,
                      7082
                    ],
                    {
                      "Ref": [
                        [
                          7063,
                          7068
                        ],
                        "foldl"
                      ]
                    },
                    {
                      "Ref": [
                        [
                          7069,
                          7073
                        ],
                        "*"
                      ]
                    }
                  ]
                },
                {
                  "Literal": [
                    [
                      7073,
                      7074
                    ],
                    {
                      "Int": 1
                    }
                  ]
                }
              ]
            },
            {
              "Ref": [
                [
                  7075,
                  7082
                ],
                "numbers"
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "List",
                [
                  {
                    "Var": "a"
                  }
                ]
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                }
              ]
            }
          ]
        },
        "name": "append",
        "patterns": [
          {
            "Var": "xs"
          },
          {
            "Var": "ys"
          }
        ],
        "expr": {
          "Case": [
            [
              7340,
              7402
            ],
            {
              "Ref": [
                [
                  7345,
                  7347
                ],
                "ys"
              ]
            },
            [
              [
                {
                  "List": []
                },
                {
                  "Ref": [
                    [
                      7367,
                      7369
                    ],
                    "xs"
                  ]
                }
              ],
              [
                "Wildcard",
                {
                  "Application": [
                    [
                      7386,
                      7402
                    ],
                    {
                      "Application": [
                        [
                          7386,
                          7402
                        ],
                        {
                          "Application": [
                            [
                              7386,
                              7402
                            ],
                            {
                              "Ref": [
                                [
                                  7386,
                                  7391
                                ],
                                "foldr"
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  7392,
                                  7396
                                ],
                                "cons"
                              ]
                            }
                          ]
                        },
                        {
                          "Ref": [
                            [
                              7397,
                              7399
                            ],
                            "ys"
                          ]
                        }
                      ]
                    },
                    {
                      "Ref": [
                        [
                          7400,
                          7402
                        ],
                        "xs"
                      ]
                    }
                  ]
                }
              ]
            ]
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "List",
                [
                  {
                    "Tag": [
                      "List",
                      [
                        {
                          "Var": "a"
                        }
                      ]
                    ]
                  }
                ]
              ]
            },
            {
              "Tag": [
                "List",
                [
                  {
                    "Var": "a"
                  }
                ]
              ]
            }
          ]
        },
        "name": "concat",
        "patterns": [
          {
            "Var": "lists"
          }
        ],
        "expr": {
          "Application": [
            [
              7556,
              7577
            ],
            {
              "Application": [
                [
                  7556,
                  7577
                ],
                {
                  "Application": [
                    [
                      7556,
                      7577
                    ],
                    {
                      "Ref": [
                        [
                          7556,
                          7561
                        ],
                        "foldr"
                      ]
                    },
                    {
                      "Ref": [
                        [
                          7562,
                          7568
                        ],
                        "append"
                      ]
                    }
                  ]
                },
                {
                  "List": [
                    [
                      7569,
                      7572
                    ],
                    []
                  ]
                }
              ]
            },
            {
              "Ref": [
                [
                  7572,
                  7577
                ],
                "lists"
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Fun": [
                {
                  "Var": "a"
                },
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "b"
                      }
                    ]
                  ]
                }
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "b"
                      }
                    ]
                  ]
                }
              ]
            }
          ]
        },
        "name": "concatMap",
        "patterns": [
          {
            "Var": "f"
          },
          {
            "Var": "list"
          }
        ],
        "expr": {
          "Application": [
            [
              7761,
              7780
            ],
            {
              "Ref": [
                [
                  7761,
                  7767
                ],
                "concat"
              ]
            },
            {
              "Application": [
                [
                  7769,
                  7779
                ],
                {
                  "Application": [
                    [
                      7769,
                      7779
                    ],
                    {
                      "Ref": [
                        [
                          7769,
                          7772
                        ],
                        "map"
                      ]
                    },
                    {
                      "Ref": [
                        [
                          7773,
                          7774
                        ],
                        "f"
                      ]
                    }
                  ]
                },
                {
                  "Ref": [
                    [
                      7775,
                      7779
                    ],
                    "list"
                  ]
                }
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Var": "a"
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                }
              ]
            }
          ]
        },
        "name": "intersperse",
        "patterns": [
          {
            "Var": "sep"
          },
          {
            "Var": "xs"
          }
        ],
        "expr": {
          "Case": [
            [
              8010,
              8200
            ],
            {
              "Ref": [
                [
                  8015,
                  8017
                ],
                "xs"
              ]
            },
            [
              [
                {
                  "List": []
                },
                {
                  "List": [
                    [
                      8037,
                      8039
                    ],
                    []
                  ]
                }
              ],
              [
                {
                  "BinaryOp": [
                    "::",
                    {
                      "Var": "hd"
                    },
                    {
                      "Var": "tl"
                    }
                  ]
                },
                {
                  "Let": [
                    [
                      8063,
                      8200
                    ],
                    [
                      {
                        "Def": {
                          "header": null,
                          "name": "step",
                          "patterns": [
                            {
                              "Var": "x"
                            },
                            {
                              "Var": "rest"
                            }
                          ],
                          "expr": {
                            "Application": [
                              [
                                8099,
                                8121
                              ],
                              {
                                "Application": [
                                  [
                                    8099,
                                    8121
                                  ],
                                  {
                                    "Ref": [
                                      [
                                        8099,
                                        8103
                                      ],
                                      "cons"
                                    ]
                                  },
                                  {
                                    "Ref": [
                                      [
                                        8104,
                                        8107
                                      ],
                                      "sep"
                                    ]
                                  }
                                ]
                              },
                              {
                                "Application": [
                                  [
                                    8109,
                                    8120
                                  ],
                                  {
                                    "Application": [
                                      [
                                        8109,
                                        8120
                                      ],
                                      {
                                        "Ref": [
                                          [
                                            8109,
                                            8113
                                          ],
                                          "cons"
                                        ]
                                      },
                                      {
                                        "Ref": [
                                          [
                                            8114,
                                            8115
                                          ],
                                          "x"
                                        ]
                                      }
                                    ]
                                  },
                                  {
                                    "Ref": [
                                      [
                                        8116,
                                        8120
                                      ],
                                      "rest"
                                    ]
                                  }
                                ]
                              }
                            ]
                          }
                        }
                      },
                      {
                        "Def": {
                          "header": null,
                          "name": "spersed",
                          "patterns": [],
                          "expr": {
                            "Application": [
                              [
                                8151,
                                8167
                              ],
                              {
                                "Application": [
                                  [
                                    8151,
                                    8167
                                  ],
                                  {
                                    "Application": [
                                      [
                                        8151,
                                        8167
                                      ],
                                      {
                                        "Ref": [
                                          [
                                            8151,
                                            8156
                                          ],
                                          "foldr"
                                        ]
                                      },
                                      {
                                        "Ref": [
                                          [
                                            8157,
                                            8161
                                          ],
                                          "step"
                                        ]
                                      }
                                    ]
                                  },
                                  {
                                    "List": [
                                      [
                                        8162,
                                        8165
                                      ],
                                      []
                                    ]
                                  }
                                ]
                              },
                              {
                                "Ref": [
                                  [
                                    8165,
                                    8167
                                  ],
                                  "tl"
                                ]
                              }
                            ]
                          }
                        }
                      }
                    ],
                    {
                      "Application": [
                        [
                          8185,
                          8200
                        ],
                        {
                          "Application": [
                            [
                              8185,
                              8200
                            ],
                            {
                              "Ref": [
                                [
                                  8185,
                                  8189
                                ],
                                "cons"
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  8190,
                                  8192
                                ],
                                "hd"
                              ]
                            }
                          ]
                        },
                        {
                          "Ref": [
                            [
                              8193,
                              8200
                            ],
                            "spersed"
                          ]
                        }
                      ]
                    }
                  ]
                }
              ]
            ]
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Fun": [
                {
                  "Var": "a"
                },
                {
                  "Fun": [
                    {
                      "Var": "b"
                    },
                    {
                      "Var": "result"
                    }
                  ]
                }
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Fun": [
                    {
                      "Tag": [
                        "List",
                        [
                          {
                            "Var": "b"
                          }
                        ]
                      ]
                    },
                    {
                      "Tag": [
                        "List",
                        [
                          {
                            "Var": "result"
                          }
                        ]
                      ]
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "map2",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              8720,
              8740
            ],
            [
              "Elm",
              "Kernel",
              "List"
            ],
            "map2"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Fun": [
                {
                  "Var": "a"
                },
                {
                  "Fun": [
                    {
                      "Var": "b"
                    },
                    {
                      "Fun": [
                        {
                          "Var": "c"
                        },
                        {
                          "Var": "result"
                        }
                      ]
                    }
                  ]
                }
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Fun": [
                    {
                      "Tag": [
                        "List",
                        [
                          {
                            "Var": "b"
                          }
                        ]
                      ]
                    },
                    {
                      "Fun": [
                        {
                          "Tag": [
                            "List",
                            [
                              {
                                "Var": "c"
                              }
                            ]
                          ]
                        },
                        {
                          "Tag": [
                            "List",
                            [
                              {
                                "Var": "result"
                              }
                            ]
                          ]
                        }
                      ]
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "map3",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              8834,
              8854
            ],
            [
              "Elm",
              "Kernel",
              "List"
            ],
            "map3"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Fun": [
                {
                  "Var": "a"
                },
                {
                  "Fun": [
                    {
                      "Var": "b"
                    },
                    {
                      "Fun": [
                        {
                          "Var": "c"
                        },
                        {
                          "Fun": [
                            {
                              "Var": "d"
                            },
                            {
                              "Var": "result"
                            }
                          ]
                        }
                      ]
                    }
                  ]
                }
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Fun": [
                    {
                      "Tag": [
                        "List",
                        [
                          {
                            "Var": "b"
                          }
                        ]
                      ]
                    },
                    {
                      "Fun": [
                        {
                          "Tag": [
                            "List",
                            [
                              {
                                "Var": "c"
                              }
                            ]
                          ]
                        },
                        {
                          "Fun": [
                            {
                              "Tag": [
                                "List",
                                [
                                  {
                                    "Var": "d"
                                  }
                                ]
                              ]
                            },
                            {
                              "Tag": [
                                "List",
                                [
                                  {
                                    "Var": "result"
                                  }
                                ]
                              ]
                            }
                          ]
                        }
                      ]
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "map4",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              8963,
              8983
            ],
            [
              "Elm",
              "Kernel",
              "List"
            ],
            "map4"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Fun": [
                {
                  "Var": "a"
                },
                {
                  "Fun": [
                    {
                      "Var": "b"
                    },
                    {
                      "Fun": [
                        {
                          "Var": "c"
                        },
                        {
                          "Fun": [
                            {
                              "Var": "d"
                            },
                            {
                              "Fun": [
                                {
                                  "Var": "e"
                                },
                                {
                                  "Var": "result"
                                }
                              ]
                            }
                          ]
                        }
                      ]
                    }
                  ]
                }
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Fun": [
                    {
                      "Tag": [
                        "List",
                        [
                          {
                            "Var": "b"
                          }
                        ]
                      ]
                    },
                    {
                      "Fun": [
                        {
                          "Tag": [
                            "List",
                            [
                              {
                                "Var": "c"
                              }
                            ]
                          ]
                        },
                        {
                          "Fun": [
                            {
                              "Tag": [
                                "List",
                                [
                                  {
                                    "Var": "d"
                                  }
                                ]
                              ]
                            },
                            {
                              "Fun": [
                                {
                                  "Tag": [
                                    "List",
                                    [
                                      {
                                        "Var": "e"
                                      }
                                    ]
                                  ]
                                },
                                {
                                  "Tag": [
                                    "List",
                                    [
                                      {
                                        "Var": "result"
                                      }
                                    ]
                                  ]
                                }
                              ]
                            }
                          ]
                        }
                      ]
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "map5",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              9107,
              9127
            ],
            [
              "Elm",
              "Kernel",
              "List"
            ],
            "map5"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "List",
                [
                  {
                    "Var": "comparable"
                  }
                ]
              ]
            },
            {
              "Tag": [
                "List",
                [
                  {
                    "Var": "comparable"
                  }
                ]
              ]
            }
          ]
        },
        "name": "sort",
        "patterns": [
          {
            "Var": "xs"
          }
        ],
        "expr": {
          "Application": [
            [
              9266,
              9284
            ],
            {
              "Application": [
                [
                  9266,
                  9284
                ],
                {
                  "Ref": [
                    [
                      9266,
                      9272
                    ],
                    "sortBy"
                  ]
                },
                {
                  "Ref": [
                    [
                      9273,
                      9281
                    ],
                    "identity"
                  ]
                }
              ]
            },
            {
              "Ref": [
                [
                  9282,
                  9284
                ],
                "xs"
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Fun": [
                {
                  "Var": "a"
                },
                {
                  "Var": "comparable"
                }
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                }
              ]
            }
          ]
        },
        "name": "sortBy",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              9693,
              9715
            ],
            [
              "Elm",
              "Kernel",
              "List"
            ],
            "sortBy"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Fun": [
                {
                  "Var": "a"
                },
                {
                  "Fun": [
                    {
                      "Var": "a"
                    },
                    {
                      "Tag": [
                        "Order",
                        []
                      ]
                    }
                  ]
                }
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                }
              ]
            }
          ]
        },
        "name": "sortWith",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              10115,
              10139
            ],
            [
              "Elm",
              "Kernel",
              "List"
            ],
            "sortWith"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "List",
                [
                  {
                    "Var": "a"
                  }
                ]
              ]
            },
            {
              "Tag": [
                "Bool",
                []
              ]
            }
          ]
        },
        "name": "isEmpty",
        "patterns": [
          {
            "Var": "xs"
          }
        ],
        "expr": {
          "Case": [
            [
              10387,
              10440
            ],
            {
              "Ref": [
                [
                  10392,
                  10394
                ],
                "xs"
              ]
            },
            [
              [
                {
                  "List": []
                },
                {
                  "Ref": [
                    [
                      10414,
                      10418
                    ],
                    "True"
                  ]
                }
              ],
              [
                "Wildcard",
                {
                  "Ref": [
                    [
                      10435,
                      10440
                    ],
                    "False"
                  ]
                }
              ]
            ]
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "List",
                [
                  {
                    "Var": "a"
                  }
                ]
              ]
            },
            {
              "Tag": [
                "Maybe",
                [
                  {
                    "Var": "a"
                  }
                ]
              ]
            }
          ]
        },
        "name": "head",
        "patterns": [
          {
            "Var": "list"
          }
        ],
        "expr": {
          "Case": [
            [
              10723,
              10788
            ],
            {
              "Ref": [
                [
                  10728,
                  10732
                ],
                "list"
              ]
            },
            [
              [
                {
                  "BinaryOp": [
                    "::",
                    {
                      "Var": "x"
                    },
                    {
                      "Var": "xs"
                    }
                  ]
                },
                {
                  "Application": [
                    [
                      10757,
                      10763
                    ],
                    {
                      "Ref": [
                        [
                          10757,
                          10762
                        ],
                        "Just"
                      ]
                    },
                    {
                      "Ref": [
                        [
                          10762,
                          10763
                        ],
                        "x"
                      ]
                    }
                  ]
                }
              ],
              [
                {
                  "List": []
                },
                {
                  "Ref": [
                    [
                      10781,
                      10788
                    ],
                    "Nothing"
                  ]
                }
              ]
            ]
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "List",
                [
                  {
                    "Var": "a"
                  }
                ]
              ]
            },
            {
              "Tag": [
                "Maybe",
                [
                  {
                    "Tag": [
                      "List",
                      [
                        {
                          "Var": "a"
                        }
                      ]
                    ]
                  }
                ]
              ]
            }
          ]
        },
        "name": "tail",
        "patterns": [
          {
            "Var": "list"
          }
        ],
        "expr": {
          "Case": [
            [
              11075,
              11141
            ],
            {
              "Ref": [
                [
                  11080,
                  11084
                ],
                "list"
              ]
            },
            [
              [
                {
                  "BinaryOp": [
                    "::",
                    {
                      "Var": "x"
                    },
                    {
                      "Var": "xs"
                    }
                  ]
                },
                {
                  "Application": [
                    [
                      11109,
                      11116
                    ],
                    {
                      "Ref": [
                        [
                          11109,
                          11114
                        ],
                        "Just"
                      ]
                    },
                    {
                      "Ref": [
                        [
                          11114,
                          11116
                        ],
                        "xs"
                      ]
                    }
                  ]
                }
              ],
              [
                {
                  "List": []
                },
                {
                  "Ref": [
                    [
                      11134,
                      11141
                    ],
                    "Nothing"
                  ]
                }
              ]
            ]
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Int",
                []
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                }
              ]
            }
          ]
        },
        "name": "take",
        "patterns": [
          {
            "Var": "n"
          },
          {
            "Var": "list"
          }
        ],
        "expr": {
          "Application": [
            [
              11267,
              11284
            ],
            {
              "Application": [
                [
                  11267,
                  11284
                ],
                {
                  "Application": [
                    [
                      11267,
                      11284
                    ],
                    {
                      "Ref": [
                        [
                          11267,
                          11275
                        ],
                        "takeFast"
                      ]
                    },
                    {
                      "Literal": [
                        [
                          11276,
                          11277
                        ],
                        {
                          "Int": 0
                        }
                      ]
                    }
                  ]
                },
                {
                  "Ref": [
                    [
                      11278,
                      11279
                    ],
                    "n"
                  ]
                }
              ]
            },
            {
              "Ref": [
                [
                  11280,
                  11284
                ],
                "list"
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Int",
                []
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Int",
                    []
                  ]
                },
                {
                  "Fun": [
                    {
                      "Tag": [
                        "List",
                        [
                          {
                            "Var": "a"
                          }
                        ]
                      ]
                    },
                    {
                      "Tag": [
                        "List",
                        [
                          {
                            "Var": "a"
                          }
                        ]
                      ]
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "takeFast",
        "patterns": [
          {
            "Var": "ctr"
          },
          {
            "Var": "n"
          },
          {
            "Var": "list"
          }
        ],
        "expr": {
          "If": [
            [
              11353,
              11825
            ],
            {
              "OpChain": [
                [
                  11356,
                  11362
                ],
                [
                  {
                    "Ref": [
                      [
                        11356,
                        11357
                      ],
                      "n"
                    ]
                  },
                  {
                    "Literal": [
                      [
                        11361,
                        11362
                      ],
                      {
                        "Int": 0
                      }
                    ]
                  }
                ],
                [
                  "<="
                ]
              ]
            },
            {
              "List": [
                [
                  11372,
                  11377
                ],
                []
              ]
            },
            {
              "Case": [
                [
                  11386,
                  11825
                ],
                {
                  "Tuple": [
                    [
                      11391,
                      11403
                    ],
                    [
                      {
                        "Ref": [
                          [
                            11393,
                            11394
                          ],
                          "n"
                        ]
                      },
                      {
                        "Ref": [
                          [
                            11396,
                            11400
                          ],
                          "list"
                        ]
                      }
                    ]
                  ]
                },
                [
                  [
                    {
                      "Tuple": [
                        "Wildcard",
                        {
                          "List": []
                        }
                      ]
                    },
                    {
                      "Ref": [
                        [
                          11433,
                          11437
                        ],
                        "list"
                      ]
                    }
                  ],
                  [
                    {
                      "Tuple": [
                        {
                          "LitInt": 1
                        },
                        {
                          "BinaryOp": [
                            "::",
                            {
                              "Var": "x"
                            },
                            "Wildcard"
                          ]
                        }
                      ]
                    },
                    {
                      "List": [
                        [
                          11470,
                          11475
                        ],
                        [
                          {
                            "Ref": [
                              [
                                11472,
                                11473
                              ],
                              "x"
                            ]
                          }
                        ]
                      ]
                    }
                  ],
                  [
                    {
                      "Tuple": [
                        {
                          "LitInt": 2
                        },
                        {
                          "BinaryOp": [
                            "::",
                            {
                              "Var": "x"
                            },
                            {
                              "BinaryOp": [
                                "::",
                                {
                                  "Var": "y"
                                },
                                "Wildcard"
                              ]
                            }
                          ]
                        }
                      ]
                    },
                    {
                      "List": [
                        [
                          11513,
                          11521
                        ],
                        [
                          {
                            "Ref": [
                              [
                                11515,
                                11516
                              ],
                              "x"
                            ]
                          },
                          {
                            "Ref": [
                              [
                                11518,
                                11519
                              ],
                              "y"
                            ]
                          }
                        ]
                      ]
                    }
                  ],
                  [
                    {
                      "Tuple": [
                        {
                          "LitInt": 3
                        },
                        {
                          "BinaryOp": [
                            "::",
                            {
                              "Var": "x"
                            },
                            {
                              "BinaryOp": [
                                "::",
                                {
                                  "Var": "y"
                                },
                                {
                                  "BinaryOp": [
                                    "::",
                                    {
                                      "Var": "z"
                                    },
                                    "Wildcard"
                                  ]
                                }
                              ]
                            }
                          ]
                        }
                      ]
                    },
                    {
                      "List": [
                        [
                          11564,
                          11575
                        ],
                        [
                          {
                            "Ref": [
                              [
                                11566,
                                11567
                              ],
                              "x"
                            ]
                          },
                          {
                            "Ref": [
                              [
                                11569,
                                11570
                              ],
                              "y"
                            ]
                          },
                          {
                            "Ref": [
                              [
                                11572,
                                11573
                              ],
                              "z"
                            ]
                          }
                        ]
                      ]
                    }
                  ],
                  [
                    {
                      "Tuple": [
                        "Wildcard",
                        {
                          "BinaryOp": [
                            "::",
                            {
                              "Var": "x"
                            },
                            {
                              "BinaryOp": [
                                "::",
                                {
                                  "Var": "y"
                                },
                                {
                                  "BinaryOp": [
                                    "::",
                                    {
                                      "Var": "z"
                                    },
                                    {
                                      "BinaryOp": [
                                        "::",
                                        {
                                          "Var": "w"
                                        },
                                        {
                                          "Var": "tl"
                                        }
                                      ]
                                    }
                                  ]
                                }
                              ]
                            }
                          ]
                        }
                      ]
                    },
                    {
                      "If": [
                        [
                          11624,
                          11800
                        ],
                        {
                          "OpChain": [
                            [
                              11627,
                              11637
                            ],
                            [
                              {
                                "Ref": [
                                  [
                                    11627,
                                    11630
                                  ],
                                  "ctr"
                                ]
                              },
                              {
                                "Literal": [
                                  [
                                    11633,
                                    11637
                                  ],
                                  {
                                    "Int": 1000
                                  }
                                ]
                              }
                            ],
                            [
                              ">"
                            ]
                          ]
                        },
                        {
                          "Application": [
                            [
                              11653,
                              11711
                            ],
                            {
                              "Application": [
                                [
                                  11653,
                                  11711
                                ],
                                {
                                  "Ref": [
                                    [
                                      11653,
                                      11657
                                    ],
                                    "cons"
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      11658,
                                      11659
                                    ],
                                    "x"
                                  ]
                                }
                              ]
                            },
                            {
                              "Application": [
                                [
                                  11661,
                                  11710
                                ],
                                {
                                  "Application": [
                                    [
                                      11661,
                                      11710
                                    ],
                                    {
                                      "Ref": [
                                        [
                                          11661,
                                          11665
                                        ],
                                        "cons"
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          11666,
                                          11667
                                        ],
                                        "y"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Application": [
                                    [
                                      11669,
                                      11709
                                    ],
                                    {
                                      "Application": [
                                        [
                                          11669,
                                          11709
                                        ],
                                        {
                                          "Ref": [
                                            [
                                              11669,
                                              11673
                                            ],
                                            "cons"
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              11674,
                                              11675
                                            ],
                                            "z"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Application": [
                                        [
                                          11677,
                                          11708
                                        ],
                                        {
                                          "Application": [
                                            [
                                              11677,
                                              11708
                                            ],
                                            {
                                              "Ref": [
                                                [
                                                  11677,
                                                  11681
                                                ],
                                                "cons"
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  11682,
                                                  11683
                                                ],
                                                "w"
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Application": [
                                            [
                                              11685,
                                              11707
                                            ],
                                            {
                                              "Application": [
                                                [
                                                  11685,
                                                  11707
                                                ],
                                                {
                                                  "Ref": [
                                                    [
                                                      11685,
                                                      11696
                                                    ],
                                                    "takeTailRec"
                                                  ]
                                                },
                                                {
                                                  "OpChain": [
                                                    [
                                                      11698,
                                                      11703
                                                    ],
                                                    [
                                                      {
                                                        "Ref": [
                                                          [
                                                            11698,
                                                            11699
                                                          ],
                                                          "n"
                                                        ]
                                                      },
                                                      {
                                                        "Literal": [
                                                          [
                                                            11702,
                                                            11703
                                                          ],
                                                          {
                                                            "Int": 4
                                                          }
                                                        ]
                                                      }
                                                    ],
                                                    [
                                                      "-"
                                                    ]
                                                  ]
                                                }
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  11705,
                                                  11707
                                                ],
                                                "tl"
                                              ]
                                            }
                                          ]
                                        }
                                      ]
                                    }
                                  ]
                                }
                              ]
                            }
                          ]
                        },
                        {
                          "Application": [
                            [
                              11735,
                              11800
                            ],
                            {
                              "Application": [
                                [
                                  11735,
                                  11800
                                ],
                                {
                                  "Ref": [
                                    [
                                      11735,
                                      11739
                                    ],
                                    "cons"
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      11740,
                                      11741
                                    ],
                                    "x"
                                  ]
                                }
                              ]
                            },
                            {
                              "Application": [
                                [
                                  11743,
                                  11799
                                ],
                                {
                                  "Application": [
                                    [
                                      11743,
                                      11799
                                    ],
                                    {
                                      "Ref": [
                                        [
                                          11743,
                                          11747
                                        ],
                                        "cons"
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          11748,
                                          11749
                                        ],
                                        "y"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Application": [
                                    [
                                      11751,
                                      11798
                                    ],
                                    {
                                      "Application": [
                                        [
                                          11751,
                                          11798
                                        ],
                                        {
                                          "Ref": [
                                            [
                                              11751,
                                              11755
                                            ],
                                            "cons"
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              11756,
                                              11757
                                            ],
                                            "z"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Application": [
                                        [
                                          11759,
                                          11797
                                        ],
                                        {
                                          "Application": [
                                            [
                                              11759,
                                              11797
                                            ],
                                            {
                                              "Ref": [
                                                [
                                                  11759,
                                                  11763
                                                ],
                                                "cons"
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  11764,
                                                  11765
                                                ],
                                                "w"
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Application": [
                                            [
                                              11767,
                                              11796
                                            ],
                                            {
                                              "Application": [
                                                [
                                                  11767,
                                                  11796
                                                ],
                                                {
                                                  "Application": [
                                                    [
                                                      11767,
                                                      11796
                                                    ],
                                                    {
                                                      "Ref": [
                                                        [
                                                          11767,
                                                          11775
                                                        ],
                                                        "takeFast"
                                                      ]
                                                    },
                                                    {
                                                      "OpChain": [
                                                        [
                                                          11777,
                                                          11784
                                                        ],
                                                        [
                                                          {
                                                            "Ref": [
                                                              [
                                                                11777,
                                                                11780
                                                              ],
                                                              "ctr"
                                                            ]
                                                          },
                                                          {
                                                            "Literal": [
                                                              [
                                                                11783,
                                                                11784
                                                              ],
                                                              {
                                                                "Int": 1
                                                              }
                                                            ]
                                                          }
                                                        ],
                                                        [
                                                          "+"
                                                        ]
                                                      ]
                                                    }
                                                  ]
                                                },
                                                {
                                                  "OpChain": [
                                                    [
                                                      11787,
                                                      11792
                                                    ],
                                                    [
                                                      {
                                                        "Ref": [
                                                          [
                                                            11787,
                                                            11788
                                                          ],
                                                          "n"
                                                        ]
                                                      },
                                                      {
                                                        "Literal": [
                                                          [
                                                            11791,
                                                            11792
                                                          ],
                                                          {
                                                            "Int": 4
                                                          }
                                                        ]
                                                      }
                                                    ],
                                                    [
                                                      "-"
                                                    ]
                                                  ]
                                                }
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  11794,
                                                  11796
                                                ],
                                                "tl"
                                              ]
                                            }
                                          ]
                                        }
                                      ]
                                    }
                                  ]
                                }
                              ]
                            }
                          ]
                        }
                      ]
                    }
                  ],
                  [
                    "Wildcard",
                    {
                      "Ref": [
                        [
                          11821,
                          11825
                        ],
                        "list"
                      ]
                    }
                  ]
                ]
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Int",
                []
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                }
              ]
            }
          ]
        },
        "name": "takeTailRec",
        "patterns": [
          {
            "Var": "n"
          },
          {
            "Var": "list"
          }
        ],
        "expr": {
          "Application": [
            [
              11888,
              11919
            ],
            {
              "Ref": [
                [
                  11888,
                  11895
                ],
                "reverse"
              ]
            },
            {
              "Application": [
                [
                  11897,
                  11918
                ],
                {
                  "Application": [
                    [
                      11897,
                      11918
                    ],
                    {
                      "Application": [
                        [
                          11897,
                          11918
                        ],
                        {
                          "Ref": [
                            [
                              11897,
                              11908
                            ],
                            "takeReverse"
                          ]
                        },
                        {
                          "Ref": [
                            [
                              11909,
                              11910
                            ],
                            "n"
                          ]
                        }
                      ]
                    },
                    {
                      "Ref": [
                        [
                          11911,
                          11915
                        ],
                        "list"
                      ]
                    }
                  ]
                },
                {
                  "List": [
                    [
                      11916,
                      11918
                    ],
                    []
                  ]
                }
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Int",
                []
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Fun": [
                    {
                      "Tag": [
                        "List",
                        [
                          {
                            "Var": "a"
                          }
                        ]
                      ]
                    },
                    {
                      "Tag": [
                        "List",
                        [
                          {
                            "Var": "a"
                          }
                        ]
                      ]
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "takeReverse",
        "patterns": [
          {
            "Var": "n"
          },
          {
            "Var": "list"
          },
          {
            "Var": "kept"
          }
        ],
        "expr": {
          "If": [
            [
              11998,
              12133
            ],
            {
              "OpChain": [
                [
                  12001,
                  12007
                ],
                [
                  {
                    "Ref": [
                      [
                        12001,
                        12002
                      ],
                      "n"
                    ]
                  },
                  {
                    "Literal": [
                      [
                        12006,
                        12007
                      ],
                      {
                        "Int": 0
                      }
                    ]
                  }
                ],
                [
                  "<="
                ]
              ]
            },
            {
              "Ref": [
                [
                  12017,
                  12021
                ],
                "kept"
              ]
            },
            {
              "Case": [
                [
                  12033,
                  12133
                ],
                {
                  "Ref": [
                    [
                      12038,
                      12042
                    ],
                    "list"
                  ]
                },
                [
                  [
                    {
                      "List": []
                    },
                    {
                      "Ref": [
                        [
                          12066,
                          12070
                        ],
                        "kept"
                      ]
                    }
                  ],
                  [
                    {
                      "BinaryOp": [
                        "::",
                        {
                          "Var": "x"
                        },
                        {
                          "Var": "xs"
                        }
                      ]
                    },
                    {
                      "Application": [
                        [
                          12097,
                          12133
                        ],
                        {
                          "Application": [
                            [
                              12097,
                              12133
                            ],
                            {
                              "Application": [
                                [
                                  12097,
                                  12133
                                ],
                                {
                                  "Ref": [
                                    [
                                      12097,
                                      12108
                                    ],
                                    "takeReverse"
                                  ]
                                },
                                {
                                  "OpChain": [
                                    [
                                      12110,
                                      12115
                                    ],
                                    [
                                      {
                                        "Ref": [
                                          [
                                            12110,
                                            12111
                                          ],
                                          "n"
                                        ]
                                      },
                                      {
                                        "Literal": [
                                          [
                                            12114,
                                            12115
                                          ],
                                          {
                                            "Int": 1
                                          }
                                        ]
                                      }
                                    ],
                                    [
                                      "-"
                                    ]
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  12117,
                                  12119
                                ],
                                "xs"
                              ]
                            }
                          ]
                        },
                        {
                          "Application": [
                            [
                              12121,
                              12132
                            ],
                            {
                              "Application": [
                                [
                                  12121,
                                  12132
                                ],
                                {
                                  "Ref": [
                                    [
                                      12121,
                                      12125
                                    ],
                                    "cons"
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      12126,
                                      12127
                                    ],
                                    "x"
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  12128,
                                  12132
                                ],
                                "kept"
                              ]
                            }
                          ]
                        }
                      ]
                    }
                  ]
                ]
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Int",
                []
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                }
              ]
            }
          ]
        },
        "name": "drop",
        "patterns": [
          {
            "Var": "n"
          },
          {
            "Var": "list"
          }
        ],
        "expr": {
          "If": [
            [
              12259,
              12374
            ],
            {
              "OpChain": [
                [
                  12262,
                  12268
                ],
                [
                  {
                    "Ref": [
                      [
                        12262,
                        12263
                      ],
                      "n"
                    ]
                  },
                  {
                    "Literal": [
                      [
                        12267,
                        12268
                      ],
                      {
                        "Int": 0
                      }
                    ]
                  }
                ],
                [
                  "<="
                ]
              ]
            },
            {
              "Ref": [
                [
                  12278,
                  12282
                ],
                "list"
              ]
            },
            {
              "Case": [
                [
                  12295,
                  12374
                ],
                {
                  "Ref": [
                    [
                      12300,
                      12304
                    ],
                    "list"
                  ]
                },
                [
                  [
                    {
                      "List": []
                    },
                    {
                      "Ref": [
                        [
                          12328,
                          12332
                        ],
                        "list"
                      ]
                    }
                  ],
                  [
                    {
                      "BinaryOp": [
                        "::",
                        {
                          "Var": "x"
                        },
                        {
                          "Var": "xs"
                        }
                      ]
                    },
                    {
                      "Application": [
                        [
                          12359,
                          12374
                        ],
                        {
                          "Application": [
                            [
                              12359,
                              12374
                            ],
                            {
                              "Ref": [
                                [
                                  12359,
                                  12363
                                ],
                                "drop"
                              ]
                            },
                            {
                              "OpChain": [
                                [
                                  12365,
                                  12370
                                ],
                                [
                                  {
                                    "Ref": [
                                      [
                                        12365,
                                        12366
                                      ],
                                      "n"
                                    ]
                                  },
                                  {
                                    "Literal": [
                                      [
                                        12369,
                                        12370
                                      ],
                                      {
                                        "Int": 1
                                      }
                                    ]
                                  }
                                ],
                                [
                                  "-"
                                ]
                              ]
                            }
                          ]
                        },
                        {
                          "Ref": [
                            [
                              12372,
                              12374
                            ],
                            "xs"
                          ]
                        }
                      ]
                    }
                  ]
                ]
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Fun": [
                {
                  "Var": "a"
                },
                {
                  "Tag": [
                    "Bool",
                    []
                  ]
                }
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Tuple": [
                    {
                      "Tag": [
                        "List",
                        [
                          {
                            "Var": "a"
                          }
                        ]
                      ]
                    },
                    {
                      "Tag": [
                        "List",
                        [
                          {
                            "Var": "a"
                          }
                        ]
                      ]
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "partition",
        "patterns": [
          {
            "Var": "pred"
          },
          {
            "Var": "list"
          }
        ],
        "expr": {
          "Let": [
            [
              12742,
              12902
            ],
            [
              {
                "Def": {
                  "header": null,
                  "name": "step",
                  "patterns": [
                    {
                      "Var": "x"
                    },
                    {
                      "Tuple": [
                        {
                          "Var": "trues"
                        },
                        {
                          "Var": "falses"
                        }
                      ]
                    }
                  ],
                  "expr": {
                    "If": [
                      [
                        12781,
                        12872
                      ],
                      {
                        "Application": [
                          [
                            12784,
                            12790
                          ],
                          {
                            "Ref": [
                              [
                                12784,
                                12788
                              ],
                              "pred"
                            ]
                          },
                          {
                            "Ref": [
                              [
                                12789,
                                12790
                              ],
                              "x"
                            ]
                          }
                        ]
                      },
                      {
                        "Tuple": [
                          [
                            12804,
                            12834
                          ],
                          [
                            {
                              "Application": [
                                [
                                  12805,
                                  12817
                                ],
                                {
                                  "Application": [
                                    [
                                      12805,
                                      12817
                                    ],
                                    {
                                      "Ref": [
                                        [
                                          12805,
                                          12809
                                        ],
                                        "cons"
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          12810,
                                          12811
                                        ],
                                        "x"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      12812,
                                      12817
                                    ],
                                    "trues"
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  12819,
                                  12825
                                ],
                                "falses"
                              ]
                            }
                          ]
                        ]
                      },
                      {
                        "Tuple": [
                          [
                            12847,
                            12872
                          ],
                          [
                            {
                              "Ref": [
                                [
                                  12848,
                                  12853
                                ],
                                "trues"
                              ]
                            },
                            {
                              "Application": [
                                [
                                  12855,
                                  12868
                                ],
                                {
                                  "Application": [
                                    [
                                      12855,
                                      12868
                                    ],
                                    {
                                      "Ref": [
                                        [
                                          12855,
                                          12859
                                        ],
                                        "cons"
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          12860,
                                          12861
                                        ],
                                        "x"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      12862,
                                      12868
                                    ],
                                    "falses"
                                  ]
                                }
                              ]
                            }
                          ]
                        ]
                      }
                    ]
                  }
                }
              }
            ],
            {
              "Application": [
                [
                  12879,
                  12902
                ],
                {
                  "Application": [
                    [
                      12879,
                      12902
                    ],
                    {
                      "Application": [
                        [
                          12879,
                          12902
                        ],
                        {
                          "Ref": [
                            [
                              12879,
                              12884
                            ],
                            "foldr"
                          ]
                        },
                        {
                          "Ref": [
                            [
                              12885,
                              12889
                            ],
                            "step"
                          ]
                        }
                      ]
                    },
                    {
                      "Tuple": [
                        [
                          12890,
                          12898
                        ],
                        [
                          {
                            "List": [
                              [
                                12891,
                                12893
                              ],
                              []
                            ]
                          },
                          {
                            "List": [
                              [
                                12894,
                                12896
                              ],
                              []
                            ]
                          }
                        ]
                      ]
                    }
                  ]
                },
                {
                  "Ref": [
                    [
                      12898,
                      12902
                    ],
                    "list"
                  ]
                }
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "List",
                [
                  {
                    "Tuple": [
                      {
                        "Var": "a"
                      },
                      {
                        "Var": "b"
                      }
                    ]
                  }
                ]
              ]
            },
            {
              "Tuple": [
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "b"
                      }
                    ]
                  ]
                }
              ]
            }
          ]
        },
        "name": "unzip",
        "patterns": [
          {
            "Var": "pairs"
          }
        ],
        "expr": {
          "Let": [
            [
              13103,
              13195
            ],
            [
              {
                "Def": {
                  "header": null,
                  "name": "step",
                  "patterns": [
                    {
                      "Tuple": [
                        {
                          "Var": "x"
                        },
                        {
                          "Var": "y"
                        }
                      ]
                    },
                    {
                      "Tuple": [
                        {
                          "Var": "xs"
                        },
                        {
                          "Var": "ys"
                        }
                      ]
                    }
                  ],
                  "expr": {
                    "Tuple": [
                      [
                        13138,
                        13163
                      ],
                      [
                        {
                          "Application": [
                            [
                              13139,
                              13148
                            ],
                            {
                              "Application": [
                                [
                                  13139,
                                  13148
                                ],
                                {
                                  "Ref": [
                                    [
                                      13139,
                                      13143
                                    ],
                                    "cons"
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      13144,
                                      13145
                                    ],
                                    "x"
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  13146,
                                  13148
                                ],
                                "xs"
                              ]
                            }
                          ]
                        },
                        {
                          "Application": [
                            [
                              13150,
                              13159
                            ],
                            {
                              "Application": [
                                [
                                  13150,
                                  13159
                                ],
                                {
                                  "Ref": [
                                    [
                                      13150,
                                      13154
                                    ],
                                    "cons"
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      13155,
                                      13156
                                    ],
                                    "y"
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  13157,
                                  13159
                                ],
                                "ys"
                              ]
                            }
                          ]
                        }
                      ]
                    ]
                  }
                }
              }
            ],
            {
              "Application": [
                [
                  13170,
                  13195
                ],
                {
                  "Application": [
                    [
                      13170,
                      13195
                    ],
                    {
                      "Application": [
                        [
                          13170,
                          13195
                        ],
                        {
                          "Ref": [
                            [
                              13170,
                              13175
                            ],
                            "foldr"
                          ]
                        },
                        {
                          "Ref": [
                            [
                              13176,
                              13180
                            ],
                            "step"
                          ]
                        }
                      ]
                    },
                    {
                      "Tuple": [
                        [
                          13181,
                          13190
                        ],
                        [
                          {
                            "List": [
                              [
                                13182,
                                13184
                              ],
                              []
                            ]
                          },
                          {
                            "List": [
                              [
                                13186,
                                13188
                              ],
                              []
                            ]
                          }
                        ]
                      ]
                    }
                  ]
                },
                {
                  "Ref": [
                    [
                      13190,
                      13195
                    ],
                    "pairs"
                  ]
                }
              ]
            }
          ]
        }
      }
    }
  ]
}